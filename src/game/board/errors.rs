use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum BoardInitializationError {
    ChainLargerThanBoard {
        board_size: usize,
        chain_size: usize,
    },
}

impl Display for BoardInitializationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ChainLargerThanBoard {
                board_size,
                chain_size,
            } => write!(
                f,
                "Invalid board configuration: chain length ({}) cannot be larger than the board size ({})",
                chain_size, board_size
            ),
        }
    }
}
