use std::{io::Read, net::{TcpListener, TcpStream}, thread};

const CHAT_URL: &str = "chat.protohackers.com";
const CHAT_PORT: u16 = 16963;
const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8888;

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT)).unwrap();
    println!("Listening on port {PORT}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("Connection failed. Error: {e}")
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buf = Vec::new();
    let _ = stream.read(&mut buf);

    let message = String::from_utf8(buf).unwrap();
    
    if message.starts_with("Hi") {
        message = 
    }
}