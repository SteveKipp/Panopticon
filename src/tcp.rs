
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::env;
use ipinfo::{IpInfo, IpInfoConfig};

pub fn listen(port: i32) {
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("----Incoming Connection----");
        connection_details(stream);
    }
}

fn connection_details(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let peer = stream.peer_addr().unwrap();

    println!("Peer: {} \n Request: {}", peer, String::from_utf8_lossy(&buffer[..]));

    addr_lookup(peer.to_string());
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
