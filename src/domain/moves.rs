#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: Option<usize>,
    pub to: Option<usize>,
    pub die: u8,
}
