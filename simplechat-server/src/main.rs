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
    println!("[INFO] Listening for connections at {}", config.server.bind_interface);
    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                let clients_collection = Arc::clone(&clients);
                thread::spawn(move || clients::handle_connection(stream, clients_collection));
            },
            Err(error) => println!("[ERROR] Failed accepting client connection: {error:?}"),
        }
    }
}
