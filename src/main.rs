use check24_ultimate_tic_tac_toe_judge::server;

#[tokio::main]
async fn main() {
    server::start("127.0.0.1:9001").await;
}
