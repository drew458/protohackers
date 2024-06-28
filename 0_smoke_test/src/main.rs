use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9876").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => { println!("connection failed") }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buf: Vec<u8> = Vec::new();

    stream.read_to_end(&mut buf).unwrap();

    stream.write_all(&buf).unwrap();
 }
