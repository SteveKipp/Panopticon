use std::net::TcpListener;

pub fn listen(port: i32) {
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection Attempt!")
    }
}
