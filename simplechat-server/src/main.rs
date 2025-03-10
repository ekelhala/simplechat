use std::{io::Read, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream}};

struct AppState {
    bind_port: u16,
    bind_addr: IpAddr
}

// start the SimpleChat server on the provided port and address
// returns the resulting listener
fn start_server(bind_port: u16, bind_addr: IpAddr) -> TcpListener {
    let addr = SocketAddr::new(bind_addr, bind_port);
    let result = TcpListener::bind(&addr);
    let listener = match result {
        Ok(listener) => listener,
        Err(error) => panic!("Could not bind to given address: {error:?}"),
    };
    listener
}

fn receive_data(buffer: &[u8;512], size: usize) {
    if size > 0 {
        let data = String::from_utf8_lossy(&buffer[..size]);
        print!("{data}");
    }
}

fn handle_connection(mut stream: TcpStream, address: SocketAddr) {
    println!("Accepted connection from {address}");
    let mut buf = [0;512];
    match stream.read(&mut buf) {
        Ok(read_bytes) => receive_data(&buf, read_bytes),
        Err(_) => (),
    };
}

fn main() {
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    println!("Starting SimpleChat server...");
    let app_state: AppState = AppState {
                                    bind_port: 21000,
                                    bind_addr: IpAddr::V4(addr)                                   
                                };
    let listener = start_server(app_state.bind_port, app_state.bind_addr);
    println!("Listening for connections");
    loop {
        match listener.accept() {
            Ok((stream, addr)) => handle_connection(stream, addr),
            Err(error) => println!("Failed accepting client connection: {error:?}"),
        }
    }
}
