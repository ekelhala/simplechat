use std::{
    io::Read,
        net::{
        IpAddr,
        SocketAddr,
        TcpListener,
        TcpStream}
    };
use serde_json::from_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Message {
    user: String,
    message: String,
    channel: String
}

// start the SimpleChat server on the provided port and address
// returns the resulting listener
pub fn start_server(bind_port: u16, bind_addr: IpAddr) -> TcpListener {
    let addr = SocketAddr::new(bind_addr, bind_port);
    let result = TcpListener::bind(&addr);
    let listener = match result {
        Ok(listener) => listener,
        Err(error) => panic!("[ERROR] Could not bind to given address: {error}"),
    };
    listener
}

pub fn handle_connection(mut stream: TcpStream, address: SocketAddr) {
    println!("[INFO] Accepted connection from {address}");
    let mut buf = [0;512];
    match stream.read(&mut buf) {
        Ok(read_bytes) => receive_data(&buf, read_bytes),
        Err(_) => (),
    };
}

// private functions

fn broadcast_message(message: Message) {
    println!("{}", message.message);
}

fn receive_data(buffer: &[u8;512], size: usize) {
    if size > 0 {
        let data = String::from_utf8_lossy(&buffer[..size]);
        match from_str(&data) {
            Ok(message) => broadcast_message(message),
            Err(e) => println!("[ERROR] Could not parse message from client: {e}")
        };
    }
}