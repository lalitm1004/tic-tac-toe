use std::num::NonZeroUsize;

use super::*;

fn create_board_with_states(board_size: usize, chain_size: usize, states: Vec<CellState>) -> Board {
    let mut board = unsafe { Board::new_unchecked(board_size, chain_size) };
    board.states = states;
    board
}

#[test]
fn test_safe_constructor_with_invalid_chain_size() {
    let board_size = NonZeroUsize::new(3).unwrap();
    let chain_size = NonZeroUsize::new(5).unwrap();

    let result = Board::new(board_size, chain_size);
    assert!(result.is_err());
}

#[test]
fn test_safe_constructor_with_valid_sizes() {
    let board_size = NonZeroUsize::new(3).unwrap();
    let chain_size = NonZeroUsize::new(3).unwrap();

    let result = Board::new(board_size, chain_size);
    assert!(result.is_ok());
}

#[test]
fn test_empty_board_no_victory() {
    let board = Board::default(); // 3x3 board
    assert!(board.is_victory_state().is_none());
}

#[test]
fn test_horizontal_victory_top_row() {
    let states = vec![
        CellState::Cross,
        CellState::Cross,
        CellState::Cross,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
    ];
    let board = create_board_with_states(3, 3, states);

    let victory = board.is_victory_state().unwrap();
    assert_eq!(victory.winner, Player::Cross);
    assert_eq!(victory.positions, vec![(0, 0), (0, 1), (0, 2)]);
}

#[test]
fn test_horizontal_victory_middle_row() {
    let states = vec![
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Circle,
        CellState::Circle,
        CellState::Circle,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
    ];
    let board = create_board_with_states(3, 3, states);

    let victory = board.is_victory_state().unwrap();
    assert_eq!(victory.winner, Player::Circle);
    assert_eq!(victory.positions, vec![(1, 0), (1, 1), (1, 2)]);
}

#[test]
fn test_vertical_victory_left_column() {
    let states = vec![
        CellState::Cross,
        CellState::Empty,
        CellState::Empty,
        CellState::Cross,
        CellState::Empty,
        CellState::Empty,
        CellState::Cross,
        CellState::Empty,
        CellState::Empty,
    ];
    let board = create_board_with_states(3, 3, states);

    let victory = board.is_victory_state().unwrap();
    assert_eq!(victory.winner, Player::Cross);
    assert_eq!(victory.positions, vec![(0, 0), (1, 0), (2, 0)]);
}

#[test]
fn test_vertical_victory_right_column() {
    let states = vec![
        CellState::Empty,
        CellState::Empty,
        CellState::Circle,
        CellState::Empty,
        CellState::Empty,
        CellState::Circle,
        CellState::Empty,
        CellState::Empty,
        CellState::Circle,
    ];
    let board = create_board_with_states(3, 3, states);

    let victory = board.is_victory_state().unwrap();
    assert_eq!(victory.winner, Player::Circle);
    assert_eq!(victory.positions, vec![(0, 2), (1, 2), (2, 2)]);
}

#[test]
fn test_diagonal_down_right_victory() {
    let states = vec![
        CellState::Cross,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Cross,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Cross,
    ];
    let board = create_board_with_states(3, 3, states);

    let victory = board.is_victory_state().unwrap();
    assert_eq!(victory.winner, Player::Cross);
    assert_eq!(victory.positions, vec![(0, 0), (1, 1), (2, 2)]);
}

#[test]
fn test_diagonal_up_right_victory() {
    let states = vec![
        CellState::Empty,
        CellState::Empty,
        CellState::Circle,
        CellState::Empty,
        CellState::Circle,
        CellState::Empty,
        CellState::Circle,
        CellState::Empty,
        CellState::Empty,
    ];
    let board = create_board_with_states(3, 3, states);

    let victory = board.is_victory_state().unwrap();
    assert_eq!(victory.winner, Player::Circle);
    assert_eq!(victory.positions, vec![(0, 2), (1, 1), (2, 0)]);
}

#[test]
fn test_no_victory_partial_board() {
    let states = vec![
        CellState::Cross,
        CellState::Circle,
        CellState::Cross,
        CellState::Circle,
        CellState::Cross,
        CellState::Circle,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
    ];
    let board = create_board_with_states(3, 3, states);
    assert!(board.is_victory_state().is_none());
}

#[test]
fn test_horizontal_victory_with_larger_board() {
    let board_size = 5;
    let chain_size = 4;
    let mut states = vec![CellState::Empty; board_size * board_size];

    // set a winning horizontal line in row 2
    for c in 1..=chain_size {
        states[2 * board_size + c] = CellState::Cross;
    }

    let board = create_board_with_states(board_size, chain_size, states);
    let victory = board.is_victory_state().unwrap();

    assert_eq!(victory.winner, Player::Cross);
    assert_eq!(victory.positions, vec![(2, 1), (2, 2), (2, 3), (2, 4)]);
}

#[test]
fn test_vertical_victory_with_larger_board() {
    let board_size = 5;
    let chain_size = 4;
    let mut states = vec![CellState::Empty; board_size * board_size];

    // set a winning vertical line in column 3
    for r in 0..chain_size {
        states[r * board_size + 3] = CellState::Circle;
    }

    let board = create_board_with_states(board_size, chain_size, states);
    let victory = board.is_victory_state().unwrap();

    assert_eq!(victory.winner, Player::Circle);
    assert_eq!(victory.positions, vec![(0, 3), (1, 3), (2, 3), (3, 3)]);
}

#[test]
fn test_diagonal_victory_with_larger_board() {
    let board_size = 5;
    let chain_size = 4;
    let mut states = vec![CellState::Empty; board_size * board_size];

    // set a winning diagonal (down-right) starting at (1, 1)
    for i in 0..chain_size {
        states[(1 + i) * board_size + (1 + i)] = CellState::Cross;
    }

    let board = create_board_with_states(board_size, chain_size, states);
    let victory = board.is_victory_state().unwrap();

    assert_eq!(victory.winner, Player::Cross);
    assert_eq!(victory.positions, vec![(1, 1), (2, 2), (3, 3), (4, 4)]);
}

#[test]
fn test_no_victory_with_broken_chain() {
    let board_size = 3;
    let chain_size = 3;

    // Almost a horizontal victory but broken in the middle
    let states = vec![
        CellState::Cross,
        CellState::Circle,
        CellState::Cross,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
        CellState::Empty,
    ];
    let board = create_board_with_states(board_size, chain_size, states);
    assert!(board.is_victory_state().is_none());
}

#[test]
fn test_victory_with_minimal_board() {
    // 1x1 board with chain size 1
    let board = unsafe { Board::new_unchecked(1, 1) };
    assert!(board.is_victory_state().is_none());

    // set the single cell to Cross
    let states = vec![CellState::Cross];
    let board = create_board_with_states(1, 1, states);
    let victory = board.is_victory_state().unwrap();

    assert_eq!(victory.winner, Player::Cross);
    assert_eq!(victory.positions, vec![(0, 0)]);
}
