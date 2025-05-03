use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerToPlayer {
    pub last_move: Option<(usize, usize)>,
    pub player: u8,
    pub board_state: [[u8; 9]; 9],
}

impl ServerToPlayer {
    pub fn decode_boards(small_boards: [u32; 9]) -> [[u8; 9]; 9] {
        let mut board_state = [[0u8; 9]; 9];

        for (board_idx, encoded) in small_boards.iter().enumerate() {
            for cell_idx in 0..9 {
                let bits = (encoded >> (cell_idx * 2)) & 0b11;
                board_state[board_idx][cell_idx] = match bits {
                    0b00 => 0,
                    0b01 => 1, // Player X
                    0b10 => 2, // Player O
                    _ => 0,    // Invalid, fallback to empty
                };
            }
        }

        board_state
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerToServer {
    #[serde(rename = "move")]
    pub coordinates: (usize, usize),
}
