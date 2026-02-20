use rusteze::server;

#[tokio::main]
async fn main() {
    server::run("127.0.0.1:6969").await;
}
