#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellState {
    Empty = 0,
    Cross = 1,
    Circle = 2,
}
