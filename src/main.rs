use std::io::prelude::*;
use std::net::{
    TcpStream,
    TcpListener,
    Shutdown,
};

const REQUEST_EOL: &str = "\r\n\r\n";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut raw_request = String::new();
    let mut buffer = [0; 128];

    println!("new connection");

    while match stream.read(&mut buffer) {
        Ok(size) => {
            let chunk = String::from_utf8_lossy(&buffer[0..size]);
            raw_request.push_str(&chunk);

            // verifica se os ultimos caracteres são REQUEST_EOL
            !chunk.contains(REQUEST_EOL)
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    let raw_response = format!("HTTP/1.1 200 OK{}", REQUEST_EOL);

    stream.write(raw_response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("disconnected");
    println!("RAW Request: {}", raw_request);
}