use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Request {
    method: String,
    number: i64
}

#[derive(Serialize, Deserialize)]
struct Response {
    method: String,
    prime: bool
}

impl Response {

    fn new(method: String, prime: bool) -> Response {
        Response {
            method,
            prime
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9876").unwrap();

    // Each connection spawns a new thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => { println!("connection failed. Error: {}", e) }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

    loop {

        let mut total_str = String::new();

        // Loop over the single request written by the connection
        loop {

            let mut buf = Vec::with_capacity(1);
            stream.read(&mut buf).unwrap();

            let received_str = String::from_utf8(buf).unwrap();

            if received_str.eq("\n") {
                break;
            } else {
                total_str.push_str(&received_str);
            }
        }

        let req: Result<Request, serde_json::Error> = serde_json::from_str(&total_str);

        match req {
            Ok(req) => {

                // Whenever you receive a conforming request, send back a correct response, and wait for another request.
                let mut res = Response::new(req.method, false);

                if is_prime(req.number){
                    res.prime = true;
                }
                
                write_res(&stream, &res);
            },
            Err(_e) => {

                // Whenever you receive a malformed request, send back a single malformed response, and disconnect the client.
                let mut res = Response::new("Malformed".into(), false);
                write_res(&stream, &res);
                return;
            }
        }
        
    }
}

fn write_res<T>(mut stream: &TcpStream, res: &T)
where
    T: ?Sized + Serialize, 
{

    let mut res_str = serde_json::to_string(res).unwrap()
            .replace("\n", "");
    res_str.push_str("\n");

    stream.write_all(res_str.as_bytes()).unwrap(); 
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
