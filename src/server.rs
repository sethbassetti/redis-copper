use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

pub const ADDRESS: &str = "127.0.0.1:6379";

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    // Store the result of the incoming stream
    let mut buf: [u8; 512] = [0; 512];

    // First, wait until something is read to the stream and then respond
    stream.read(&mut buf)?;

    // After getting a request, send the response to the client
    stream.write_all(b"+PONG\r\n")?;

    Ok(())
}

pub fn run_server() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    // Process connections serially
    for stream in listener.incoming() {
        println!("Accepted new connection");
        let mut stream = stream.unwrap();

        handle_connection(&mut stream).expect("Error writing to stream");
    }
}
