#[macro_use]
extern crate rocket;
use check24_ultimate_tic_tac_toe_judge::protocol::ServerToPlayer;
use check24_ultimate_tic_tac_toe_judge::team2::test_clients;
use dashmap::DashMap;
use rand::Rng;
use rocket::State;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_cors::{Cors, CorsOptions};
use std::sync::Arc;

use check24_ultimate_tic_tac_toe_judge::{BoardState, UltimateTicTacToe};

type LobbyToGameMap = Arc<DashMap<u32, UltimateTicTacToe>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateGameRequest {
    enemy: String,
    name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateGameResponse {
    game_id: u32,
    player: u8,
}

#[put("/api/create_game", data = "<request>")]
fn create_game(
    map: &State<LobbyToGameMap>,
    request: Json<CreateGameRequest>,
) -> Json<CreateGameResponse> {
    let game = UltimateTicTacToe::new(request.name.clone());
    let mut rng = rand::rng();
    let game_id = rng.random::<u32>();
    map.insert(game_id, game);

    let player_id = rng.random_range(1..=2);
    println!(
        "Game created with ID: {} and player: {}",
        game_id, player_id
    );
    Json(CreateGameResponse {
        game_id,
        player: player_id,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Lobby {
    lobby_id: u32,
    name: String,
}

#[get("/api/lobbies")]
fn get_lobbies(map: &State<LobbyToGameMap>) -> Json<Vec<Lobby>> {
    let mut lobbies = Vec::new();
    for entry in map.iter() {
        let (id, game) = (entry.key(), entry.value());
        lobbies.push(Lobby {
            lobby_id: *id,
            name: game.name.clone(),
        });
    }
    Json(lobbies)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameStateResponse {
    status: String,
    last_move: Option<(u8, u8)>,
    player: Option<u8>,
    board: [[u8; 9]; 9],
    winner: Option<u8>,
}

#[get("/api/game_state/<game_id>")]
fn get_game_state(map: &State<LobbyToGameMap>, game_id: u32) -> Json<Option<GameStateResponse>> {
    match map.get(&game_id) {
        Some(game) => {
            let state = game.check_winner();
            let winner: Option<u8> = match game.global_state {
                BoardState::InProgress => None,
                BoardState::Won(player) => Some(player as u8),
                BoardState::Draw => Some(0),
            };
            Json(Some(GameStateResponse {
                status: match game.global_state {
                    BoardState::InProgress => "running",
                    BoardState::Won(_) => "won",
                    BoardState::Draw => "draw",
                }
                .to_string(),
                last_move: game.last_move,
                player: Some(game.current_player as u8),
                board: ServerToPlayer::decode_boards(game.small_boards),
                winner,
            }))
        }
        None => Json(None),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MakeMoveRequest {
    player: u8,
    coords: [u8; 2],
}

#[post("/api/make_move/<game_id>", data = "<move_data>")]
fn make_move(
    map: &State<LobbyToGameMap>,
    game_id: u32,
    move_data: Json<MakeMoveRequest>,
) -> Json<Result<(), String>> {
    match map.get_mut(&game_id) {
        Some(mut game) => match game.play(move_data.coords[0], move_data.coords[1]) {
            Ok(_) => Json(Ok(())),
            Err(err) => Json(Err(err.to_string())),
        },
        None => Json(Err("Game not found".to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    // test_clients();
    let lobby_to_game_map: LobbyToGameMap = Arc::new(DashMap::new());
    rocket::build()
        .attach(CorsOptions::default().to_cors().unwrap())
        .manage(lobby_to_game_map)
        .mount(
            "/",
            routes![get_lobbies, create_game, get_game_state, make_move],
        )
}
