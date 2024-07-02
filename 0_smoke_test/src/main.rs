use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

/**
 * This is a very stupid implementation. The server spawns a new thread for every connecction that arrives without reusing any.
 */
fn main() {
    let addr = "0.0.0.0";
    let port = 8888_u16;

    let listener = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();
    println!("Listening on port {port}");

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