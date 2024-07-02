fn main() {
    let listener = TcpListener::bind("127.0.0.1:9876").unwrap();

    // Each connection spawns a new thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("connection failed. Error: {}", e)
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    

}
