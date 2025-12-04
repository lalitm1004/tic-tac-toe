mod errors;
pub use errors::BoardInitializationError;

use std::num::NonZeroUsize;

use crate::game::{cell::CellState, player::Player};

#[derive(Debug, Clone)]
pub struct Board {
    board_size: NonZeroUsize,
    chain_size: NonZeroUsize,

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
            board_size,
            chain_size,
            states: vec![CellState::Empty; board_size.into()],
            next_turn: Player::Cross,
        })
    }

    #[inline]
    const fn pos_to_idx(&self, r: usize, c: usize) -> usize {
        let size = self.board_size.get();
        r * size + c
    }

    #[inline]
    const fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        let size = self.board_size.get();
        (idx / size, idx % size)
    }

    #[inline]
    const fn in_bounds(&self, r: usize, c: usize) -> bool {
        let size = self.board_size.get();
        r < size && c < size
    }

    pub fn is_victory_state(&self) -> Option<VictoryInfo> {
        todo!()
    }
}

const DEFAULT_BOARD_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(3) };
const DEFAULT_CHAIN_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(3) };
impl Default for Board {
    fn default() -> Self {
        Self {
            board_size: DEFAULT_BOARD_SIZE,
            chain_size: DEFAULT_CHAIN_SIZE,
            states: vec![CellState::Empty; DEFAULT_BOARD_SIZE.into()],
            next_turn: Player::Cross,
        }
    }
}
