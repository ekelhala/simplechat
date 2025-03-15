use std::
        net::{
        IpAddr,
        SocketAddr,
        TcpListener};

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
