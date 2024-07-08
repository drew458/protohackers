use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    str::from_utf8,
    sync::{Arc, RwLock},
    thread,
};

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8888;

fn main() {
    let socket = UdpSocket::bind(format!("{}:{}", ADDR, PORT)).unwrap();
    println!("Listening on port {PORT}");
    let _ = socket.set_read_timeout(None); // Read calls will block indefinitely

    let db: Arc<RwLock<HashMap<String, String>>> = Arc::from(RwLock::new(HashMap::new()));

    loop {
        let mut buf = Vec::new();
        let (_, src) = socket.recv_from(&mut buf).unwrap();

        let shared_db = db.clone();
        let socket_clone = socket.try_clone().expect("couldn't clone the socket");

        thread::spawn(move || execute_request(buf, socket_clone, src, shared_db));
    }
}

fn execute_request(
    buf: Vec<u8>,
    socket: UdpSocket,
    src: SocketAddr,
    db: Arc<RwLock<HashMap<String, String>>>,
) {
    let message = from_utf8(&buf).unwrap();

    // Insert
    if message.contains("=") {
        let (key, val) = message.split_once("=").unwrap();

        db.write().unwrap().insert(key.to_string(), val.to_string());
    } else {
        // Retrieve
        let value = db.read().unwrap().get(message).cloned().unwrap();

        let response_message: &str = &format!("{message}={value}");

        let buf: &[u8] = response_message.as_bytes();
        let _ = socket.send_to(buf, src);
    }
}
