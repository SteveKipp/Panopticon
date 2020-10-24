use iced::futures;
use tokio::{
    stream::Stream,
    net::TcpListener,
};
use std::{env};
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

//Stop Note - This stream instead of reaching the optional Some(()), is getting a function back '()'
pub fn listen(addr: String) -> impl Stream<Item = Connection> {
    futures::stream::unfold(State::Active, |state| async move {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        match state{
            State::Active => {

                match listener.accept().await {
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
