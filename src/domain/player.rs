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

    pub fn sign(self) -> i8 {
        if self == Self::White { 1 } else { -1 }
    }

    pub fn in_home(self, i: usize) -> bool {
        if self == Self::White { i < 6 } else { i >= 18 }
    }
}
