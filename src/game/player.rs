#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Cross = 0,
    Circle = 1,
}

impl Player {
    pub const fn other(&self) -> Self {
        match self {
            Self::Cross => Self::Circle,
            Self::Circle => Self::Cross,
        }
    }
}
