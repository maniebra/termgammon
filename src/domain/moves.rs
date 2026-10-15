#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: Option<usize>,
    pub to: Option<usize>,
    pub die: u8,
}

pub fn without(dice: &[u8], die: u8) -> Vec<u8> {
    let mut rest = dice.to_vec();
    rest.remove(rest.iter().position(|&d| d == die).unwrap());
    rest
}
