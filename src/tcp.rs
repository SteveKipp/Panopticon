use chrono;

use iced::futures;
use std::{env, fmt};
use futures::Stream;
use std::net::TcpListener;
use ipinfo::{IpInfo, IpInfoConfig};



#[derive(Debug, Clone, Copy)]
pub enum Connection {
    New(std::net::SocketAddr),
    Err,
}


pub struct ConnectionDetails {
    pub addr: String,
    pub hostname: String,
    pub city: String,
    pub country: String,
    pub region: String,
    pub timezone: String,
    pub location: String,
    pub timestamp: String,
}

enum State{
    Active,
    Stopped,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Connection::New(addr) => addr.to_string(),
            Connection::Err => "Error obtaining incoming address".to_string(),
        };
        write!(f, "{}", printable)
    }
}

pub fn listen(addr: &'static str) -> impl Stream<Item = Connection> {
    futures::stream::unfold(State::Active, move |state| async move {
        let listener = TcpListener::bind(addr).unwrap();
        match state{
            State::Active => {

                match listener.accept(){
                    Ok((_socket, addr)) => Some((Connection::New(addr), State::Active)),
                    Err(e) => Some((Connection::Err, State::Stopped)),
                }
            },
            State::Stopped => Some((Connection::Err, State::Stopped)),
        }
    })
}

pub fn addr_lookup(addr: String) ->  ConnectionDetails{
    let key = env::var("IPINFO_KEY").unwrap();
    let config = IpInfoConfig { token: Some(key.to_string()), ..Default::default() };
    let mut ipinfo = IpInfo::new(config).expect("should construct");
    let ip = &addr[0..addr.find(':').unwrap()];
    let timestamp = format!("{:?}", chrono::offset::Local::now());
    let res = ipinfo.lookup(&[ip]);

    match res {

        Ok(r) => {
            println!("---- Peer Address Lookup ---");

            let details = &r[ip];
            println!("Hostname: {}", details.hostname);
            println!("City: {}", details.city);
            println!("Country: {}", details.country);
            println!("Region: {}", details.region);
            println!("Lat, Long: {}", details.loc);
            let timezone_str = format!("{:?}", details.timezone);
            println!("Timezone: {}", timezone_str);
            println!("Timestamp: {}", timestamp);
            println!("\n\n");

            ConnectionDetails{
                addr: ip.to_string(),
                hostname: details.hostname.clone(),
                city: details.city.clone(),
                country: details.country.clone(),
                region: details.region.clone(),
                location: details.loc.clone(),
                timezone: timezone_str,
                timestamp: timestamp,
            }
        },
        Err(e) => {
            ConnectionDetails{
                addr: ip.to_string(),
                hostname: "!ERR".to_string(),
                city: "!ERR".to_string(),
                country: "!ERR".to_string(),
                region: "!ERR".to_string(),
                location: "!ERR".to_string(),
                timezone: "!ERR".to_string(),
                timestamp: timestamp,
            }

        },
    }
}
