use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

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
    println!("Peer: {} \n Details: {}", stream.peer_addr().unwrap(),
                                        String::from_utf8_lossy(&buffer[..]));

}
