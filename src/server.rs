use futures_util::{SinkExt, StreamExt};
use rand::random;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::game::UltimateTicTacToe;
use crate::{Player, protocol::*};

pub async fn start(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {addr}, waiting for 2 players...");

    let (s1, _) = listener.accept().await.unwrap();
    let mut ws1 = accept_async(s1).await.unwrap();
    println!("Player 1 connected");

    let (s2, _) = listener.accept().await.unwrap();
    let mut ws2 = accept_async(s2).await.unwrap();
    println!("Player 2 connected");

    loop {
        println!("Both players connected. Starting game.");

        // Randomly swap players
        if random::<bool>() {
            run_game(&mut ws1, &mut ws2, false).await;
        } else {
            run_game(&mut ws2, &mut ws1, true).await;
        }
    }
}

async fn run_game<S>(mut p1: S, mut p2: S, swapped_players: bool)
where
    S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
        + SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
        + Unpin,
{
    let mut game = UltimateTicTacToe::new(String::new());
    let mut last_move: Option<(usize, usize)> = None;

    let mut winner: Option<Player> = None;

    while winner.is_none() {
        for player in [&mut p1, &mut p2] {
            game.print_board();
            let msg = ServerToPlayer {
                last_move,
                player: game.current_player as u8,
                board_state: ServerToPlayer::decode_boards(game.small_boards),
            };
            let txt = serde_json::to_string(&msg).unwrap();
            player.send(Message::Text(txt.into())).await.unwrap();

            let recv = player.next().await.unwrap().unwrap();
            if let Message::Text(text) = recv {
                let m: PlayerToServer = serde_json::from_str(&text).unwrap();
                match game.play(m.coordinates.0 as u8, m.coordinates.1 as u8) {
                    Ok(_) => {}
                    Err(err) => {
                        player
                            .send(Message::Text(format!("Invalid move: {}", err).into()))
                            .await
                            .unwrap();

                        winner = match game.current_player {
                            Player::X => Some(Player::O),
                            Player::O => Some(Player::X),
                        };
                        break;
                    }
                }
                last_move = Some(m.coordinates);
            }

            if let Ok(winner_of_game) = game.check_winner() {
                winner = Some(winner_of_game);
                break;
            }

            if let Err("Draw") = game.check_winner() {
                println!("Draw");
                return;
            }
        }
    }
    if swapped_players {
        winner = match winner {
            Some(Player::X) => Some(Player::O),
            Some(Player::O) => Some(Player::X),
            None => None,
        };
    }
    println!(
        "Winner: {}",
        match winner.unwrap() {
            Player::X => "Player 1",
            Player::O => "Player 2",
        }
    );
    p1.send(Message::Text(
        format!(
            "Winner: {}",
            match winner.unwrap() {
                Player::X => "X",
                Player::O => "O",
            }
        )
        .into(),
    ))
    .await
    .unwrap();
    p2.send(Message::Text(format!("Winner: {}", winner.unwrap()).into()))
        .await
        .unwrap();
}
