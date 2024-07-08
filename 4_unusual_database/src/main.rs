use std::{collections::HashMap, net::UdpSocket, str::from_utf8, sync::{Arc, RwLock}};

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8888;

fn main() {
    let socket = UdpSocket::bind(format!("{}:{}", ADDR, PORT)).unwrap();
    println!("Listening on port {PORT}");

    let db: Arc<RwLock<HashMap<String, String>>> = Arc::from(RwLock::new(HashMap::new()));

    let mut buf = Vec::new();
    let _ = socket.recv(&mut buf);

    let message = from_utf8(&buf).unwrap();

    // Insert
    if message.contains("=") {
        let parts = message.split_once("=").unwrap();

        db.write().unwrap().insert(parts.0.to_string(), parts.1.to_string());


    } else {    // Retrieve
        let value = db.read().unwrap().get(message).cloned().unwrap();

        let response_message: &str = &format!("{message}={value}");

        let buf: &[u8] = response_message.as_bytes();
        let _ = socket.send(buf);
    }
}
