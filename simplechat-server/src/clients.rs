// function implementations for client handler thread

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{
    Arc,
    Mutex, MutexGuard
};
use std::collections::HashMap;

use serde_json::{from_str, json, to_string, to_value, Value};
use serde::{Deserialize, Serialize};

// types and structs
pub type ClientsCollection = Arc<Mutex<HashMap<String, ClientConnection>>>;

#[derive(Deserialize, Serialize)]
struct Message {
    user: String,
    message: String,
    channel: String
}

pub struct ClientConnection {
    // holds the client's TCP connection and channels
    // they are interested in
    stream: TcpStream,
    channels: Vec<String>
}

// public interface
pub fn handle_connection(stream: TcpStream, clients_collection:ClientsCollection) {
    let addr = stream.peer_addr().unwrap().to_string();
    println!("[INFO] Accepted connection from {addr}");
    let mut reader = BufReader::new(&stream);

    {
        let mut clients_lock = clients_collection.lock().unwrap();
        clients_lock.insert(addr.clone(), ClientConnection {
            stream: stream.try_clone().unwrap(),
            channels: Vec::new()
        });
    }

    let mut line = String::new();
    while let Ok(bytes) = reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        let message = format!("{}\n", line.trim());
        let clients_lock = clients_collection.lock().unwrap();
        handle_message(&message, clients_lock, &addr);
        line.clear();
    }

    {
        let mut clients_lock = clients_collection.lock().unwrap();
        clients_lock.remove(&addr);
    }

    println!("Client {} disconnected", addr);

}

//private functions

fn handle_message(message: &String,
                  mut clients_lock: MutexGuard<HashMap<String, ClientConnection>>, 
                  my_addr: &String) {

    let message_value: Value = match from_str(message) {
        Ok(value) => value,
        Err(e) => {
            println!("Could not parse client message: {}", e);
            Value::Null
        }
    };
    if message_value != Value::Null {
        // checking if we have "message"-field -> it is a message
        if message_value["message"] != Value::Null {
            broadcast_message(Message {
                message: message_value["message"].to_string(),
                user: message_value["user"].to_string(),
                channel: message_value["channel"].to_string()
            }, clients_lock, my_addr);
        }
        // if we have "channels"-field -> user wants to set channels
        else if message_value["channel"] != Value::Null {
            let client: &mut ClientConnection = clients_lock.get_mut(my_addr).unwrap();
            client.channels.push(message_value["channel"].to_string());

        }
        // if none of the above -> invalid message
        else {
            println!("[ERROR]: Got invalid message: {}", message_value.to_string());
        }
    }
}

fn broadcast_message(message: Message, 
                     clients_lock: MutexGuard<HashMap<String, ClientConnection>>,
                     my_addr: &String) {
    
    println!("Got message {} from user {}", message.message, message.user);
    let payload = to_string(&message).unwrap();
    let bytes = payload.as_bytes();
    for (client_addr, client_connection) in clients_lock.iter() {
        if client_addr != my_addr && client_connection.channels.contains(&message.channel) {
            match client_connection.stream.try_clone().unwrap().write(bytes) {
                Ok(_) => println!("[INFO] broadcasted message"),
                Err(_) => println!("[ERROR] failed to broadcast message"),
            }
        }
    }
}