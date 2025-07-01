use redis_copper::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new(None).await.unwrap();
    server.run().await.unwrap();
}
