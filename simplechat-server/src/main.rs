mod server;
mod clients;
mod config_parser;

use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

use clients::ClientsCollection;

fn main() {
    println!("[INFO] Starting SimpleChat server...");
    let clients:ClientsCollection = Arc::new(Mutex::new(HashMap::new()));
    let config = config_parser::load_config();
    let listener = server::start_server(config.server.bind_port, config.server.bind_interface);
    println!("[INFO] Listening for connections at {}:{}", 
            config.server.bind_interface,
            config.server.bind_port);
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                if clients.lock().unwrap().len() < config.server.max_clients.into() {
                    let clients_collection = Arc::clone(&clients);
                    thread::spawn(move || clients::handle_connection(stream, clients_collection));
                }
                else {
                    println!("[INFO] Rejected client {addr}, because exceeded server.max_clients")
                }
            },
            Err(error) => println!("[ERROR] Failed accepting client connection: {error:?}"),
        }
    }
}
