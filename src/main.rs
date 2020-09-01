// use std::io::{
//     prelude::*,
//     ErrorKind,
// };
// use std::net::{
//     TcpStream,
//     TcpListener,
//     Shutdown,
// };
// use std::time::Duration;

mod lib;
use lib::App;

fn main() {
    let mut my_app = App::new("lala".to_string());
    my_app.parse_request("GET /index.html HTTP/1.1".to_string());

    my_app.middleware(move | req, _res | {
        println!("Middleware 1 {:?}", req);
    });
    my_app.middleware(move | req, _res | {
        println!("Middleware 2 {:?}", req);
    });

    my_app.call();
    println!("{}", my_app);

    // let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    // // implementar async IO (Epoll, Kqueue and IOCP)
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(s) => {
    //             handle_request(s);
    //         }
    //         Err(e) => panic!("encountered IO error: {}", e),
    //     }
    // }
}

// fn handle_request(mut stream: TcpStream) {
//     let mut buffer = [0; 128];
//     let mut raw_request = String::new();

//     stream.set_nodelay(true).expect("failed to set nodelay");
//     stream.set_nonblocking(false).expect("failed to set nonblocking");
//     stream.set_read_timeout(Some(Duration::new(0, 100))).expect("failed to set read timeout");

//     loop {
//         match stream.read(&mut buffer[..]) {
//             Ok(size) => {
//                 let chunk = String::from_utf8_lossy(&buffer[0..size]);
//                 raw_request.push_str(&chunk);
//                 println!("Received: {} {:?}", size, chunk);

//                 if size == 0 {
//                     break;
//                 }
//             }
//             Err(e) if e.kind() == ErrorKind::WouldBlock => {
//                 eprintln!("WouldBlock: {}", e);
//                 break;
//             }
//             Err(e) if e.kind() == ErrorKind::Interrupted => {
//                 println!("Interrupted: {}", e);
//                 break;
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//                 break;
//             }
//         }
//     }{}
    
//     println!("{}", raw_request);

//     stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\n\r\n{ \"ok\": true }\r\n\r\n").unwrap();
//     stream.flush().unwrap();
//     stream.shutdown(Shutdown::Both).expect("shutdown call failed");

// }
