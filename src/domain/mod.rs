mod board;

pub use board::Board;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn idx(self) -> usize {
        self as usize
    }

    fn sign(self) -> i8 {
        if self == Self::White { 1 } else { -1 }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: Option<usize>,
    pub to: Option<usize>,
    pub die: u8,
}
