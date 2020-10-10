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

    //stop note:  how to conver SocketAddr to &str
    addr_lookup(peer.to_str());
}

fn addr_lookup(addr: &str) {
    let key = env::var("IPINFO_KEY").unwrap();
    let config = IpInfoConfig { token: Some(key.to_string()), ..Default::default() };
    let mut ipinfo = IpInfo::new(config).expect("should construct");
    let res = ipinfo.lookup(&[addr]);

    println!("---- Peer Address Lookup ---");
    match res {
        Ok(r) => println!("{}: {}", addr, r[addr].hostname),
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
