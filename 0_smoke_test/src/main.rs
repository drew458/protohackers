use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8888;

/**
 * This is a very stupid implementation. The server spawns a new thread for every connecction that arrives without reusing any.
 */
fn main() {
    let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT)).unwrap();
    println!("Listening on port {PORT}");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("connection failed. Error: {e}")
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf: Vec<u8> = Vec::new();

    stream.read_to_end(&mut buf).unwrap();

    stream.write_all(&buf).unwrap();
}