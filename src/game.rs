pub struct UltimateTicTacToe {
    board: [[Option<char>; 9]; 9],
    current: char,
}

impl UltimateTicTacToe {
    pub fn new() -> Self {
        Self {
            board: [[None; 9]; 9],
            current: 'X',
        }
    }

    pub fn play_move(&mut self, mv: (usize, usize)) -> bool {
        let (x, y) = mv;
        if x >= 9 || y >= 9 || self.board[x][y].is_some() {
            return false;
        }
        self.board[x][y] = Some(self.current);
        self.current = if self.current == 'X' { 'O' } else { 'X' };
        true
    }

    pub fn check_winner(&self) -> Option<char> {
        // TODO: implement full UTTT win check
        None
    }
}
