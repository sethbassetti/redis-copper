#[tokio::main]
async fn main() {
    redis_copper::server::run_server().await;
}
