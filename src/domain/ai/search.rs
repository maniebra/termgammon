use crate::domain::{Board, Move, Player, without};

pub fn pick(b: &Board, p: Player, dice: &[u8], depth: u8) -> Option<Move> {
    b.legal_moves(p, dice).into_iter().max_by_key(|&m| after(b, p, dice, m, depth))
}

fn after(b: &Board, p: Player, dice: &[u8], m: Move, depth: u8) -> i32 {
    let mut next = b.clone();
    next.apply(p, m);
    score(&next, p, &without(dice, m.die), depth - 1)
}

fn score(b: &Board, p: Player, dice: &[u8], depth: u8) -> i32 {
    let moves = if depth == 0 { vec![] } else { b.legal_moves(p, dice) };
    moves.into_iter().map(|m| after(b, p, dice, m, depth)).max().unwrap_or_else(|| b.eval(p))
}
