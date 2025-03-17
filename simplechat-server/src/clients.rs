// function implementations for client handler thread

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{
    Arc,
    Mutex, MutexGuard
};
use std::collections::HashMap;

use serde_json::{from_str, from_value, to_string, Value};
use serde::{Deserialize, Serialize};

// types and structs
pub type ClientsCollection = Arc<Mutex<HashMap<String, ClientConnection>>>;
const MESSAGE_TYPES: [&str; 3] = ["message", "join_channel", "leave_channel"];

pub struct ClientConnection {
    // holds the client's TCP connection and channels
    // they are interested in
    stream: TcpStream,
    channels: Vec<String>
}

#[derive(Deserialize, Serialize)]
struct Message {
    message_type: String,
    user: String,
    data: String,
    channel: String
}

#[derive(Deserialize, Serialize)]
struct ChannelSettingsMessage {
    message_type: String,
    channel: String
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

    match from_str(message) {
        Ok(value) => {
            match get_message_type(&value) {
                Some(client_message_type) => {
                    if client_message_type == "message" {
                        // parse Message
                        match from_value(value) {
                            Ok(message) => broadcast_message(message, clients_lock, my_addr),
                            Err(e) => println!("[ERROR] Got invalid Message: {}", e)
                        }
                    }
                    else if client_message_type == "join_channel" || client_message_type == "leave_channel" {
                        // parse ChannelSettingsMessage
                        match from_value(value) {
                            Ok(channel_settings_message) => {
                                let client = clients_lock.get_mut(my_addr).unwrap();
                                if client_message_type == "join_channel" {
                                    add_channel_to_connection(channel_settings_message, client);
                                }
                                else {
                                    remove_channel_from_connection(channel_settings_message, client);
                                }
                            },
                            Err(e) => println!("[ERROR] Got invalid ChannelSettingsMessage: {}", e)
                        }
                    }
                }
                None => println!("[ERROR]: Got invalid message: {}", value.to_string())
            }
        }
    Err(e) => println!("[ERROR] Could not parse client message: {}", e)
    }
}

fn broadcast_message(message: Message,
                     clients_lock: MutexGuard<HashMap<String, ClientConnection>>,
                     my_addr: &String) {
    
    println!("Got message {} from user {}", message.data, message.user);
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

fn get_message_type(message_value: &Value) -> Option<String> {
    let message_type = message_value["message_type"].as_str().unwrap();
    if MESSAGE_TYPES.contains(&message_type) {
        let msg = message_value["message_type"].as_str();
        return Some(msg.unwrap().to_string());
    }
    None
}

fn add_channel_to_connection(channel_message: ChannelSettingsMessage, client_connection: &mut ClientConnection) {
    client_connection.channels.push(channel_message.channel);
}

fn remove_channel_from_connection(channel_message: ChannelSettingsMessage, client_connection: &mut ClientConnection) {
    let index = client_connection.channels.binary_search(&channel_message.channel).unwrap();
    client_connection.channels.swap_remove(index);
}