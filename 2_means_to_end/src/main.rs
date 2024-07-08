use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

struct Tick {
    timestamp: u32,
    price: i32,
}

impl Tick {
    fn new(timestamp: u32, price: i32) -> Tick {
        Tick { timestamp, price }
    }
}

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
                println!("connection failed. Error: {e}")
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut ticks = Vec::new();

    loop {
        let mut buf: [u8; 9] = [0; 9];
        let _ = stream.read(&mut buf); // read 9 bytes
        let first_char = buf[0].to_ascii_uppercase();

        match first_char {
            73 => {
                // I - Insert operation
                let ts: &[u8; 4] = buf[1..4].try_into().unwrap();
                let price: &[u8; 4] = buf[4..9].try_into().unwrap();
                insert(ts, price, &mut ticks)
            }
            81 => {
                // Q - Query operation
                let min_time: &[u8; 4] = buf[1..4].try_into().unwrap();
                let max_time: &[u8; 4] = buf[4..9].try_into().unwrap();

                let res_buf = query(min_time, max_time, &ticks);
                stream.write_all(&res_buf).unwrap();
            }
            _ => {
                // Behaviour is undefined if the type specifier is not either 'I' or 'Q'.
                stream
                    .shutdown(std::net::Shutdown::Both)
                    .expect("TCP shutdown call failed")
            }
        }
    }
}

fn insert(timestamp: &[u8; 4], price: &[u8; 4], prices: &mut Vec<Tick>) {
    let ts = u32::from_be_bytes(timestamp.to_owned());
    let price: i32 = i32::from_be_bytes(price.to_owned());

    let tick = Tick::new(ts, price);

    prices.push(tick);
}

fn query(min_time: &[u8; 4], max_time: &[u8; 4], prices: &[Tick]) -> [u8; 4] {
    let min_time = u32::from_be_bytes(min_time.to_owned());
    let max_time = u32::from_be_bytes(max_time.to_owned());

    // If mintime comes after maxtime the value returned must be 0.
    if min_time > max_time {
        return 0_i32.to_be_bytes();
    }

    let mut prices_sum: i32 = 0;
    let mut count: i32 = 0;

    for price in prices.iter() {
        if min_time <= price.timestamp && price.timestamp >= max_time {
            prices_sum += price.price;
            count += 1;
        }
    }

    // If there are no samples within the requested period the value returned must be 0.
    if count == 0 {
        return 0_i32.to_be_bytes();
    }

    let mean = prices_sum / count;
    mean.to_be_bytes()
}
