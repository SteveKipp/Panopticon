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

fn addr_lookup(addr: String) {
    let key = env::var("IPINFO_KEY").unwrap();
    let config = IpInfoConfig { token: Some(key.to_string()), ..Default::default() };
    let mut ipinfo = IpInfo::new(config).expect("should construct");
    let ip = &addr[0..addr.find(':').unwrap()];
    let res = ipinfo.lookup(&[ip]).expect("should lookup");

    println!("---- Peer Address Lookup ---");

    let details = &res[ip];
    println!("City: {}", details.city);
    println!("Country: {}", details.country);
    println!("Region: {}", details.region);
    println!("Lat, Long: {}", details.loc);
    println!("Timezone: {:?}", details.timezone);

    println!("\n\n");
}
