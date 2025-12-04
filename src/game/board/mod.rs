mod errors;
pub use errors::BoardInitializationError;

use std::{num::NonZeroUsize, rc::Rc};

use crate::game::{cell::CellState, player::Player};

#[derive(Debug, Clone)]
pub struct Board {
    board_size: usize,
    chain_size: usize,

    states: Vec<CellState>,

    next_turn: Player,

    horizontal_mask: Rc<[bool]>,
    vertical_mask: Rc<[bool]>,
    diagonal_1_mask: Rc<[bool]>,
    diagonal_2_mask: Rc<[bool]>,
}

#[derive(Debug)]
pub struct VictoryInfo {
    pub winner: Player,
    pub positions: Vec<(usize, usize)>,
}

impl Board {
    const DIR_HORIZONTAL: (isize, isize) = (0, 1);
    const DIR_VERTICAL: (isize, isize) = (1, 0);
    const DIR_DIAGONAL_1: (isize, isize) = (1, 1);
    const DIR_DIAGONAL_2: (isize, isize) = (1, -1);

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

        Ok(unsafe { Self::new_unchecked(board_size.into(), chain_size.into()) })
    }

    pub unsafe fn new_unchecked(board_size: usize, chain_size: usize) -> Self {
        let board_size_val = board_size;
        let chain_size_val = chain_size;
        let total_cells = board_size_val * board_size_val;

        let horizontal_mask =
            Self::generate_mask(board_size_val, chain_size_val, Board::DIR_HORIZONTAL);
        let vertical_mask =
            Self::generate_mask(board_size_val, chain_size_val, Board::DIR_VERTICAL);
        let diagonal_1_mask =
            Self::generate_mask(board_size_val, chain_size_val, Board::DIR_DIAGONAL_1);
        let diagonal_2_mask =
            Self::generate_mask(board_size_val, chain_size_val, Board::DIR_DIAGONAL_2);

        Self {
            board_size: board_size_val,
            chain_size: chain_size_val,
            states: vec![CellState::Empty; total_cells],
            next_turn: Player::Cross,
            horizontal_mask,
            vertical_mask,
            diagonal_1_mask,
            diagonal_2_mask,
        }
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
        // horizontal
        for (idx, &should_check) in self.horizontal_mask.iter().enumerate() {
            if should_check {
                let (r, c) = self.idx_to_pos(idx);
                if let Some(info) = self.check_chain(r, c, Board::DIR_HORIZONTAL) {
                    return Some(info);
                }
            }
        }

        // vertical
        for (idx, &should_check) in self.vertical_mask.iter().enumerate() {
            if should_check {
                let (r, c) = self.idx_to_pos(idx);
                if let Some(info) = self.check_chain(r, c, Board::DIR_VERTICAL) {
                    return Some(info);
                }
            }
        }

        // diagonal 1
        for (idx, &should_check) in self.diagonal_1_mask.iter().enumerate() {
            if should_check {
                let (r, c) = self.idx_to_pos(idx);
                if let Some(info) = self.check_chain(r, c, Board::DIR_DIAGONAL_1) {
                    return Some(info);
                }
            }
        }

        // diagonal 2
        for (idx, &should_check) in self.diagonal_2_mask.iter().enumerate() {
            if should_check {
                let (r, c) = self.idx_to_pos(idx);
                if let Some(info) = self.check_chain(r, c, Board::DIR_DIAGONAL_2) {
                    return Some(info);
                }
            }
        }

        None
    }

    fn generate_mask(board_size: usize, chain_size: usize, dir: (isize, isize)) -> Rc<[bool]> {
        let total_cells = board_size * board_size;
        let mut mask = vec![false; total_cells];

        let (dr, dc) = dir;

        for r in 0..board_size {
            for c in 0..board_size {
                let end_r = r as isize + dr * (chain_size as isize - 1);
                let end_c = c as isize + dc * (chain_size as isize - 1);

                if end_r >= 0
                    && end_r < board_size as isize
                    && end_c >= 0
                    && end_c < board_size as isize
                {
                    let idx = r * board_size + c;
                    mask[idx] = true;
                }
            }
        }

        mask.into()
    }

    fn check_chain(&self, r: usize, c: usize, dir: (isize, isize)) -> Option<VictoryInfo> {
        let first_state = self.states[self.pos_to_idx(r, c)];
        if first_state == CellState::Empty {
            return None;
        }

        let (dr, dc) = dir;

        let mut positions = Vec::with_capacity(self.chain_size);
        positions.push((r, c));

        for i in 1..self.chain_size {
            let new_r = (r as isize + dr * i as isize) as usize;
            let new_c = (c as isize + dc * i as isize) as usize;

            if !self.in_bounds(new_r, new_c) {
                return None;
            }

            let idx = self.pos_to_idx(new_r, new_c);

            if self.states[idx] != first_state {
                return None;
            }

            positions.push((new_r, new_c));
        }

        let winner = match first_state {
            CellState::Cross => Player::Cross,
            CellState::Circle => Player::Circle,
            CellState::Empty => unreachable!(),
        };

        Some(VictoryInfo { winner, positions })
    }
}

impl Default for Board {
    #[inline]
    fn default() -> Self {
        const BOARD_SIZE: usize = 3;
        const CHAIN_SIZE: usize = 3;

        unsafe { Self::new_unchecked(BOARD_SIZE, CHAIN_SIZE) }
    }
}

#[cfg(test)]
mod test;
