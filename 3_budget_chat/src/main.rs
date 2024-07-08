use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    sync::{Arc, RwLock},
    thread,
};

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8888;

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT)).unwrap();
    println!("Listening on port {PORT}");

    let users_map = Arc::from(RwLock::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let shared_stream = Arc::from(RwLock::new(stream)).clone();
                let shared_users_map = users_map.clone();

                thread::spawn(|| client_connected(shared_stream, shared_users_map));
            }
            Err(e) => {
                println!("Connection failed. Error: {e}")
            }
        }
    }
}

fn client_connected(
    stream: Arc<RwLock<TcpStream>>,
    users: Arc<RwLock<HashMap<String, Arc<RwLock<TcpStream>>>>>,
) {
    let _ = stream
        .write()
        .unwrap()
        .write_all("Welcome to budgetchat! What shall I call you?".as_bytes());

    let mut buf = Vec::new();
    let _ = stream.write().unwrap().read(&mut buf);

    // Now, a name has been written. Must allow at least 16 characters
    let user_name = from_utf8(&buf).unwrap();

    // If the user requests an illegal name, the server may send an informative error message to the client, and the server must disconnect the client
    if user_name.len() > 100 {
        let _ = stream
            .write()
            .unwrap()
            .write("Name must not be over 100 characters".as_bytes());
        let _ = stream.write().unwrap().shutdown(std::net::Shutdown::Both);
        return;
    }

    // The server must send the new user a message that lists all present users' names, not including the new user
    let mut buf: String = "* The room contains: ".into();
    for (i, item) in users.read().unwrap().keys().enumerate() {
        buf.push_str(item);

        if i != users.read().unwrap().len() - 1 {
            // This is not the last element
            buf.push(',');
        }
    }
    let _ = stream.write().unwrap().write(buf.as_bytes());

    // Send notification to all the connected clients that the new user has entered the room
    for (_k, v) in users.write().unwrap().iter() {
        let _ = v
            .write()
            .unwrap()
            .write(format!("* {user_name} has entered the room").as_bytes());
    }

    // Register the user
    users
        .write()
        .unwrap()
        .insert(user_name.to_string(), stream.clone());

    // From now on, chat messages will be received
    loop {
        let mut buf = Vec::new();
        match stream.write().unwrap().read(&mut buf) {
            Ok(bytes) => {
                // If 0 bytes are read, the connection has been shut down by the client
                if bytes == 0 {
                    users.write().unwrap().remove(user_name);

                    // Send message to all the connected clients that the user has left the chat
                    for (k, v) in users.write().unwrap().iter() {
                        if *k == user_name {
                            continue;
                        }

                        let _ = v
                            .write()
                            .unwrap()
                            .write(format!("* {user_name} has left the room").as_bytes());

                        return;
                    }
                }
            }

            Err(e) => {
                if e.kind() == ErrorKind::Interrupted {
                    continue; // Retry
                } else {
                    users.write().unwrap().remove(user_name);

                    // Send message to all the connected clients that the user has left the chat
                    for (k, v) in users.write().unwrap().iter() {
                        if *k == user_name {
                            continue;
                        }

                        let _ = v
                            .write()
                            .unwrap()
                            .write(format!("* {user_name} has left the room").as_bytes());
                    }

                    return;
                }
            }
        }

        let message = from_utf8(&buf).unwrap();

        // Send message to the other connected clients
        for (k, v) in users.write().unwrap().iter() {
            if *k == user_name {
                continue;
            }

            let _ = v
                .write()
                .unwrap()
                .write(format!("[{k}] {message}").as_bytes());
        }
    }
}
