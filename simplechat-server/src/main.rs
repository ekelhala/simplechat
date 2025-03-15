use std::net::{
        IpAddr,
        Ipv4Addr
    };

mod server;
struct AppState {
    bind_port: u16,
    bind_addr: IpAddr
}

fn main() {
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    println!("[INFO] Starting SimpleChat server...");
    let app_state: AppState = AppState {
                                    bind_port: 21000,
                                    bind_addr: IpAddr::V4(addr)                                   
                                };
    let listener = server::start_server(app_state.bind_port, app_state.bind_addr);
    println!("[INFO] Listening for connections");
    loop {
        match listener.accept() {
            Ok((stream, addr)) => server::handle_connection(stream, addr),
            Err(error) => println!("[ERROR] Failed accepting client connection: {error:?}"),
        }
    }
}
