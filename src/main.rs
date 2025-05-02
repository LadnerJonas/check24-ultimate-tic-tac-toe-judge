mod game;
mod protocol;
mod server;

#[tokio::main]
async fn main() {
    server::start("127.0.0.1:9001").await;
}

