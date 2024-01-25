use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use crate::web_sockets;
#[allow(non_snake_case)]
pub fn TCPClient(time_in_seconds: u32) -> f64 {
    let mut average_of_btc: f64 = 0.00;
    match TcpStream::connect("0.0.0.0:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let total_values: (f64, u32, f64) = web_sockets::btc_data(time_in_seconds);
            let average: f64 = total_values.2;
            average_of_btc = average;
            let average_bytes = average.to_ne_bytes();

            stream.write(&average_bytes).unwrap();
            println!("Aggregated data sent,awaiting for response...");

            let mut data = [0 as u8; 8]; // using 8 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == &average_bytes {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect, maybe the server isn't online: {}", e);
        }
    }
    println!("Thank you.");
    average_of_btc
}
