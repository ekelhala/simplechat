mod server;
mod clients;

use std::{collections::HashMap, net::{
        IpAddr,
        Ipv4Addr
    }, sync::{Arc, Mutex}, thread};

use clients::ClientsCollection;

struct AppState {
    bind_port: u16,
    bind_addr: IpAddr
}

fn main() {
    println!("[INFO] Starting SimpleChat server...");
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let clients:ClientsCollection = Arc::new(Mutex::new(HashMap::new()));
    let app_state: AppState = AppState {
                                    bind_port: 21000,
                                    bind_addr: IpAddr::V4(addr)                                   
                                };
    let listener = server::start_server(app_state.bind_port, app_state.bind_addr);
    println!("[INFO] Listening for connections");
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
