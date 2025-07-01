use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};

/// The default address to use for a Redis Server
const DEFAULT_ADDRESS: &str = "127.0.0.1:6379";

pub struct Server {
    pub listener: TcpListener,
}

impl Server {
    /// Given an address (or None), finds an available port and creates a TCP Listener
    pub async fn new(address: Option<&str>) -> Result<Self> {
        // Unwrap the address and bind a new tcp listener
        let address = address.unwrap_or(DEFAULT_ADDRESS).to_string();
        let listener = TcpListener::bind(address).await?;

        // Assign the actual address the listener is bound to
        Ok(Self { listener: listener })
    }

    /// Runs the
    pub async fn run(&self) -> Result<()> {
        // Handle incoming client connections asynchronously
        loop {
            // If a client connection fails, just continue to the next client
            let (stream, _) = match self.listener.accept().await {
                Ok(val) => val,
                Err(_) => continue,
            };

            // Each client connection requires a separate async task
            // So that we can have thousands of simultaneous clients
            // If the server is waiting to read a client request, it can do other tasks concurrently
            tokio::spawn(async move {
                // If an error occurs during the connection, print to stderr and wait for next client
                if let Err(e) = Server::handle_connection(stream).await {
                    eprintln!("Connection error: {:?}", e);
                }
            });
        }
    }

    /// Handles a single client connection by reading input and responding in a loop
    async fn handle_connection(mut stream: TcpStream) -> Result<()> {
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
}
