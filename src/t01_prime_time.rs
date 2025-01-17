use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Request {
    method: String,
    number: Number
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct Response {
    method: String,
    prime: bool
}

fn is_prime(number: Number) -> bool {
    if !number.is_u64() {
        return false;
    }
    primes::is_prime(number.as_u64().unwrap())
}

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut line: String = String::new();
    let ip = stream.peer_addr()?.ip().to_string();
    println!("New connection from {}", ip);
    let mut reader = BufReader::new(stream.try_clone()?);
    while reader.read_line(&mut line).is_ok() && !line.is_empty() {
        println!("Received line: {}", &line);
        match serde_json::from_str::<Request>(&*line) {
            Ok(request) if request.method == "isPrime" => {
                let response = Response {
                    method: String::from("isPrime"),
                    prime: is_prime(request.number)
                };
                let mut response_json = serde_json::to_vec(&response).expect("serialization failed");
                response_json.push(b'\n');
                stream.write(&*response_json).expect("write failed");
            },
            _ => {
                stream.write("invalid request\n".as_ref()).expect("write failed");
                break;
            }
        };
        line.clear();
    }
    stream.flush().expect("flush failed");
    stream.shutdown(Shutdown::Both).expect("shutdown failed");
    println!("Connection closed: {ip}");
    Ok(())
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    println!("running prime time");
    let listener = TcpListener::bind("0.0.0.0:10001")?;
    for stream in listener.incoming() {
        tokio::spawn(handle_client(stream?));
    }
    Ok(())
}
