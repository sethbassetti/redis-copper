use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

struct Server {
    child: Child,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn ping(stream: &mut TcpStream) -> Result<String, Error> {
    // Send the RESP-encoded PING to the server
    stream.write_all(b"*1\r\n$4\r\nPING\r\n")?;

    // Get the response back
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}

fn start_server() -> (Server, TcpStream) {
    let child = Command::new("cargo")
        .args(["run", "--quiet"])
        .spawn()
        .expect("Cargo failed to compile server");

    // Give server a moment to start
    // Give it a moment to start up
    thread::sleep(Duration::from_millis(200));

    // Then, connect to the server
    let stream = TcpStream::connect(redis_copper::server::ADDRESS).unwrap();
    (Server { child }, stream)
}
#[test]
fn test_ping() {
    let (_server, mut stream) = start_server();

    let response = ping(&mut stream).unwrap();

    // Assert the response matches the Redis "+PONG\r\n" format
    assert_eq!(response, "+PONG\r\n");
}
