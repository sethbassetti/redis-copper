use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};

pub const ADDRESS: &str = "127.0.0.1:6379";

async fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    loop {
        // Store the result of the incoming stream
        let mut buf: [u8; 512] = [0; 512];

        // First, wait until something is read to the stream and then respond
        let bytes_read = stream.read(&mut buf).await?;

        if bytes_read == 0 {
            break;
        }

        // After getting a request, send the response to the client
        stream.write_all(b"+PONG\r\n").await?;
    }

    Ok(())
}

pub async fn run_server() {
    let listener = TcpListener::bind(ADDRESS).await.unwrap();

    loop {
        let (mut stream, _) = match listener.accept().await {
            Ok(val) => val,
            Err(_) => continue,
        };
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut stream).await {
                eprintln!("Connection error: {:?}", e);
            }
        });
    }
}
