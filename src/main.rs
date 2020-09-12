// mod lib;
// use lib::{App, Request, Response};
use std::{env, error::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use std::net::Shutdown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:9000".to_string());
    let mut server = TcpListener::bind(&addr).await?;

    println!("listening: {}", addr);

    loop {
        let (stream, _) = server.accept().await?;

        println!("accepted {:?}", stream.peer_addr());

        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                eprintln!("failed to process connection; err = {}", e);
            }
        });
    }
}

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 2048];
    let mut raw_request = String::new();

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size == 0 => {
                // println!("socket closed");
                break;
            },
            Ok(size) => {
                let chunk = String::from_utf8_lossy(&buffer[0..size]);
                raw_request.push_str(&chunk);
            },
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return Err(e.into());
            },
        };

        if let Err(e) = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\n\r\n{ \"ok\": true }\r\n\r\n").await {
            eprintln!("failed to write to socket; err = {:?}", e);
        }

        stream.flush().await?;
        stream.shutdown(Shutdown::Write)?;
    };

    Ok(())
}
