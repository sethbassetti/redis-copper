use std::io::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Duration;

use redis_copper::server::Server;

const FREE_ADDRESS: &str = "127.0.0.1:0";

async fn connect_client() -> TcpStream {
    let address = begin_server().await;
    let stream = TcpStream::connect(address).await.unwrap();
    stream
}
async fn ping(stream: &mut TcpStream) -> Result<String, Error> {
    // Send the RESP-encoded PING to the server
    stream.write_all(b"*1\r\n$4\r\nPING\r\n").await?;

    // Get the response back
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).await?;
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}

async fn echo(stream: &mut TcpStream, message: &str) -> Result<String, Error> {
    let message = format!("*1\r\n$4\r\n{}\r\n", message);
    let message = message.as_bytes();

    // Send the RESP-encoded PING to the server
    stream.write_all(message).await?;

    // Get the response back
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).await?;
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}
async fn begin_server() -> SocketAddr {
    let server = Server::new(Some(FREE_ADDRESS)).await.unwrap();
    let address = server.listener.local_addr().unwrap();

    // Run the server in the background
    tokio::spawn(async move {
        server.run().await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(10)).await;
    address
}

/// Simple test to see if we can start a server and connect to it
#[tokio::test]
async fn test_connect() {
    connect_client().await;
}

/// Test whether a ping command gets a PONG response
#[tokio::test]
async fn test_ping() {
    // Open up a stream and send a ping
    let mut stream = connect_client().await;
    let response = ping(&mut stream).await.unwrap();

    // Assert the response matches the Redis "+PONG\r\n" format
    assert_eq!(response, "+PONG\r\n");
}

#[tokio::test]
async fn test_pings() {
    let mut stream = connect_client().await;

    // Test that the server can respond to multiple requests in one connection
    for _ in 0..3 {
        let response = ping(&mut stream).await.unwrap();
        assert_eq!(response, "+PONG\r\n");
    }
}

#[tokio::test]
async fn test_concurrent_connections() {
    let address = begin_server().await;

    // Define how many concurrent clients to simulate
    let num_clients = 100;

    // Create a vector of tasks
    let mut handles = Vec::with_capacity(num_clients);

    // Spawn num_clients as separate tokio tasks
    for _ in 0..num_clients {
        // Have each client send a PING and wait for a response for the server
        let handle = tokio::spawn(async move {
            let mut stream = TcpStream::connect(address)
                .await
                .expect("Failed to connect");
            let response = ping(&mut stream).await.expect("Failed to ping");
            assert_eq!(response, "+PONG\r\n");
        });
        handles.push(handle);
    }

    // Await all tasks to finish
    for handle in handles {
        handle.await.expect("Task panicked");
    }
}

#[tokio::test]
async fn test_echo() {
    let mut stream = connect_client().await;

    let response = echo(&mut stream, "Hey").await.unwrap();
    assert_eq!(response, "3\r\nHey\r\n");

    let response = echo(&mut stream, "\"Hello World\"").await.unwrap();
    assert_eq!(response, "3\r\nHello World\r\n");
}
