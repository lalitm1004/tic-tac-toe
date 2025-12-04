use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::NonZeroUsize,
};

#[derive(Debug)]
pub enum BoardInitializationError {
    ChainLargerThanBoard {
        board_size: NonZeroUsize,
        chain_size: NonZeroUsize,
    },
}

impl Error for BoardInitializationError {}

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
