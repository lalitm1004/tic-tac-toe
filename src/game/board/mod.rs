mod errors;
pub use errors::BoardInitializationError;

use crate::game::{cell::CellState, player::Player};

#[derive(Debug, Clone)]
pub struct Board {
    board_size: usize,
    chain_size: usize,

    states: Vec<CellState>,

    next_turn: Player,
}

#[derive(Debug)]
pub struct VictoryInfo {
    pub winner: Player,
    pub positions: Vec<(usize, usize)>,
}

impl Board {
    pub fn new(board_size: usize, chain_size: usize) -> Result<Self, BoardInitializationError> {
        if chain_size > board_size {
            return Err(BoardInitializationError::ChainLargerThanBoard {
                board_size,
                chain_size,
            });
        }

        Ok(Self {
            board_size,
            chain_size,
            states: vec![CellState::Empty; board_size],
            next_turn: Player::Cross,
        })
    }

    #[inline]
    fn pos_to_idx(&self, r: usize, c: usize) -> usize {
        r * self.board_size + c
    }

    #[inline]
    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx / self.board_size, idx % self.board_size)
    }

    pub fn is_victory_state(&self) -> Option<VictoryInfo> {
        todo!()
    }
}

const DEFAULT_BOARD_SIZE: usize = 3;
const DEFAULT_CHAIN_SIZE: usize = 3;
impl Default for Board {
    fn default() -> Self {
        Self {
            board_size: DEFAULT_BOARD_SIZE,
            chain_size: DEFAULT_CHAIN_SIZE,
            states: vec![CellState::Empty; DEFAULT_BOARD_SIZE],
            next_turn: Player::Cross,
        }
    }
}
