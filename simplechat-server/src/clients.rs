// function implementations for client handler thread

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{
    Arc,
    Mutex, MutexGuard
};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

// types and structs
pub type ClientsCollection = Arc<Mutex<HashMap<String, TcpStream>>>;

#[derive(Deserialize, Serialize)]
struct Message {
    user: String,
    message: String,
    channel: String
}

// public interface
pub fn handle_connection(stream: TcpStream, clients_collection:ClientsCollection) {
    let addr = stream.peer_addr().unwrap().to_string();
    println!("[INFO] Accepted connection from {addr}");
    let mut reader = BufReader::new(&stream);

    {
        let mut clients_lock = clients_collection.lock().unwrap();
        clients_lock.insert(addr.clone(), stream.try_clone().unwrap());
    }

    let mut line = String::new();
    while let Ok(bytes) = reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        let message = format!("{}\n", line.trim());
        let clients_lock = clients_collection.lock().unwrap();
        match from_str(&message) {
            Ok(message) => {
                broadcast_message(message, clients_lock, &addr);
            },
            Err(e) => println!("Could not parse message: {}", e)
        };
        line.clear();
    }

    {
        let mut clients_lock = clients_collection.lock().unwrap();
        clients_lock.remove(&addr);
    }

    println!("Client {} disconnected", addr);

}

//private functions

fn broadcast_message(message: Message, 
                     clients_lock: MutexGuard<HashMap<String, TcpStream>>,
                     my_addr: &String) {
    
    println!("Got message {} from user {}", message.message, message.user);
    let payload = to_string(&message).unwrap();
    let bytes = payload.as_bytes();
    for (client_addr, client_stream) in clients_lock.iter() {
        if client_addr != my_addr {
            match client_stream.try_clone().unwrap().write(bytes) {
                Ok(_) => println!("[INFO] broadcasted message"),
                Err(_) => println!("[ERROR] failed to broadcast message"),
            }
        }
    }
}