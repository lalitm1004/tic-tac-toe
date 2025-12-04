mod errors;
pub use errors::BoardInitializationError;

use std::num::NonZeroUsize;

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
    pub fn new(
        board_size: NonZeroUsize,
        chain_size: NonZeroUsize,
    ) -> Result<Self, BoardInitializationError> {
        if chain_size > board_size {
            return Err(BoardInitializationError::ChainLargerThanBoard {
                board_size,
                chain_size,
            });
        }

        Ok(Self {
            board_size: board_size.into(),
            chain_size: chain_size.into(),
            states: vec![CellState::Empty; board_size.into()],
            next_turn: Player::Cross,
        })
    }

    #[inline]
    const fn pos_to_idx(&self, r: usize, c: usize) -> usize {
        r * self.board_size + c
    }

    #[inline]
    const fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx / self.board_size, idx % self.board_size)
    }

    #[inline]
    const fn in_bounds(&self, r: usize, c: usize) -> bool {
        r < self.board_size && c < self.board_size
    }

    pub fn is_victory_state(&self) -> Option<VictoryInfo> {
        todo!()
    }
}

impl Default for Board {
    fn default() -> Self {
        const DEFAULT_BOARD_SIZE: usize = 3;
        const DEFAULT_CHAIN_SIZE: usize = 3;

        Self {
            board_size: DEFAULT_BOARD_SIZE,
            chain_size: DEFAULT_CHAIN_SIZE,
            states: vec![CellState::Empty; DEFAULT_BOARD_SIZE],
            next_turn: Player::Cross,
        }
    }
}
