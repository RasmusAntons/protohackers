use std::io::{Read, Write};
use std::vec::Vec;
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buf: Vec<u8> = Vec::new();
    let ip = stream.peer_addr().unwrap().ip().to_string();
    println!("New connection from {}", ip);
    if stream.read_to_end(&mut buf).is_ok() {
        let n = buf.len();
        println!("received {n}: {}", String::from_utf8_lossy(&buf));
        stream.write(&buf).expect("write failed");
        stream.flush().expect("flush failed");
        stream.shutdown(Shutdown::Both).expect("shutdown failed");
        println!("wrote to stream");
    } else {
        println!("Connection lost");
    }
}

pub fn main() -> std::io::Result<()> {
    // test with "tcpspray -4e 3po.ch 10001"
    println!("running smoke test");
    let listener = TcpListener::bind("0.0.0.0:10001")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
