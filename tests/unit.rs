use check24_ultimate_tic_tac_toe_judge::{Player, UltimateTicTacToe, game::BoardState};

#[test]
fn test_small_board_win_with_forced_board() {
    let mut board = UltimateTicTacToe::new();

    assert!(board.play(0, 0).is_ok());
    assert!(board.play(1, 1).is_ok());

    assert!(board.play(5, 3).is_ok());
    assert!(board.play(6, 0).is_ok());

    assert!(board.play(0, 1).is_ok());
    assert!(board.play(0, 3).is_ok());

    assert!(board.play(0, 2).is_ok());
    assert_eq!(board.small_board_state[0], BoardState::Won(Player::X));
}

#[test]
fn test_invalid_move_on_won_board() {
    let mut board = UltimateTicTacToe::new();
    // Simulate board 0 being won
    assert!(board.play(0, 0).is_ok());
    board.small_board_state[0] = BoardState::Won(Player::X);
    assert!(!board.play(0, 1).is_ok()); // invalid move into won board
}

#[test]
fn test_win_board_win_with_forced_board() {
    let mut board = UltimateTicTacToe::new();

    assert!(board.play(0, 0).is_ok());
    assert!(board.play(1, 1).is_ok());

    assert!(board.play(5, 3).is_ok());
    assert!(board.play(6, 0).is_ok());

    assert!(board.play(0, 1).is_ok());
    assert!(board.play(0, 3).is_ok());

    assert!(board.play(0, 2).is_ok());
    assert_eq!(board.small_board_state[0], BoardState::Won(Player::X));
    assert!(board.play(0, 6).is_ok());

    assert!(board.play(4, 4).is_ok());
    assert!(board.play(5, 5).is_ok());

    assert!(board.play(8, 8).is_ok());
    assert!(board.play(8, 7).is_ok());

    assert!(board.play(8, 4).is_ok());
    assert!(board.play(7, 4).is_ok());

    assert!(board.play(3, 5).is_ok());
    assert_eq!(board.small_board_state[4], BoardState::Won(Player::X));
    assert!(board.play(2, 8).is_ok());

    assert!(board.play(7, 7).is_ok());
    assert!(board.play(5, 8).is_ok());

    assert!(board.check_winner().is_err());

    assert!(board.play(6, 6).is_ok());
    assert_eq!(board.small_board_state[8], BoardState::Won(Player::X));
    assert_eq!(board.check_winner(), Ok(Player::X));
}

#[test]
fn test_print_board() {
    let mut board = UltimateTicTacToe::new();
    println!("{:?}", board);
    assert!(board.play(0, 0).is_ok());
    assert!(board.play(1, 1).is_ok());

    board.print_board();
}

#[test]
fn test_draw() {
    let mut board = UltimateTicTacToe::new();

    assert!(board.play(0, 0).is_ok());
    assert!(board.play(1, 1).is_ok());

    assert!(board.play(5, 3).is_ok());
    assert!(board.play(6, 0).is_ok());

    assert!(board.play(0, 1).is_ok());
    assert!(board.play(0, 3).is_ok());

    assert!(board.play(0, 2).is_ok());
    assert_eq!(board.small_board_state[0], BoardState::Won(Player::X));
    assert!(board.play(0, 6).is_ok());

    assert!(board.play(4, 4).is_ok());
    assert!(board.play(5, 5).is_ok());

    assert!(board.play(8, 8).is_ok());
    assert!(board.play(8, 7).is_ok());

    assert!(board.play(8, 4).is_ok());
    assert!(board.play(7, 4).is_ok());

    assert!(board.play(3, 5).is_ok());
    assert_eq!(board.small_board_state[4], BoardState::Won(Player::X));
    assert!(board.play(2, 8).is_ok());

    assert!(board.play(7, 7).is_ok());
    assert!(board.play(5, 8).is_ok());

    assert!(board.check_winner().is_err());

    assert!(board.play(6, 7).is_ok());
    assert!(board.play(0, 5).is_ok());

    assert!(board.play(0, 8).is_ok());
    assert!(board.play(1, 7).is_ok());
    assert_eq!(board.small_board_state[2], BoardState::Won(Player::O));

    assert!(board.play(3, 8).is_ok());
    assert!(board.play(0, 4).is_ok());
    assert_eq!(board.small_board_state[1], BoardState::Won(Player::O));

    assert!(board.play(4, 7).is_ok());
    assert!(board.play(4, 8).is_ok());

    assert!(board.play(5, 6).is_ok());
    assert_eq!(board.small_board_state[5], BoardState::Won(Player::X));
    assert!(board.play(7, 1).is_ok());

    assert!(board.play(8, 0).is_ok());
    assert!(board.play(8, 2).is_ok());
    assert_eq!(board.small_board_state[6], BoardState::Won(Player::O));

    assert!(board.play(6, 8).is_ok());
    assert!(board.play(6, 6).is_ok());

    assert!(board.play(7, 6).is_ok());
    assert!(board.play(4, 1).is_ok());

    assert!(board.play(3, 2).is_ok());
    assert!(board.play(3, 0).is_ok());

    assert!(board.play(3, 1).is_ok());
    assert!(board.play(7, 8).is_ok());
    // assert!(board.play(5, 2).is_ok());
    // assert_eq!(board.small_board_state[3], BoardState::Won(Player::O));
    assert!(board.play(4, 2).is_ok());
    assert!(board.play(8, 6).is_ok());

    assert!(board.play(4, 0).is_ok());
    assert!(board.play(5, 2).is_ok());

    assert!(board.play(6, 3).is_ok());
    assert!(board.play(8, 3).is_ok());

    assert!(board.play(6, 5).is_ok());
    assert!(board.play(6, 4).is_ok());

    assert!(board.play(7, 3).is_ok());
    assert!(board.play(7, 5).is_ok());

    assert!(board.play(8, 5).is_ok());

    assert_eq!(board.check_winner(), Err("Draw"));
    board.print_board();
}
