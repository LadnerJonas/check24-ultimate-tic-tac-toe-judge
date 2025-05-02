use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::game::UltimateTicTacToe;
use crate::protocol::*;

pub async fn start(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {addr}, waiting for 2 players...");

    let (s1, _) = listener.accept().await.unwrap();
    let ws1 = accept_async(s1).await.unwrap();
    println!("Player 1 connected");

    let (s2, _) = listener.accept().await.unwrap();
    let ws2 = accept_async(s2).await.unwrap();
    println!("Player 2 connected");

    println!("Both players connected. Starting game.");
    run_game(ws1, ws2).await;
}

async fn run_game<S>(mut p1: S, mut p2: S)
where
    S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
        + SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
        + Unpin,
{
    let mut game = UltimateTicTacToe::new();
    let mut last_move: Option<(usize, usize)> = None;

    loop {
        for player in [&mut p1, &mut p2] {
            let msg = ServerToPlayer { last_move };
            let txt = serde_json::to_string(&msg).unwrap();
            player.send(Message::Text(txt.into())).await.unwrap();

            let recv = player.next().await.unwrap().unwrap();
            if let Message::Text(text) = recv {
                let m: PlayerToServer = serde_json::from_str(&text).unwrap();
                if !game.play_move(m.mv) {
                    player
                        .send(Message::Text("Invalid move".to_string().into()))
                        .await
                        .unwrap();
                    return;
                }
                last_move = Some(m.mv);
            }

            if let Some(winner) = game.check_winner() {
                player
                    .send(Message::Text(format!("{winner} wins").into()))
                    .await
                    .unwrap();
                return;
            }
        }
    }
}
