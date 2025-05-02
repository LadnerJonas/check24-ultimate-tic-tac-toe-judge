use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerToPlayer {
    pub last_move: Option<(usize, usize)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerToServer {
    #[serde(rename = "move")]
    pub coordinates: (usize, usize),
}
