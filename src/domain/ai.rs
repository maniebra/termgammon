use super::{Board, Move, Player};

// ponytail: greedy one-move lookahead, search full dice sequences if it plays too weak
pub fn best(b: &Board, p: Player, dice: &[u8]) -> Option<Move> {
    b.legal_moves(p, dice).into_iter().max_by_key(|&m| {
        let mut next = b.clone();
        next.apply(p, m);
        next.eval(p)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits_blot() {
        let mut b = Board::default();
        b.points[1] = 1;
        b.points[5] = 4;
        let m = best(&b, Player::Black, &[1]).unwrap();
        assert_eq!(m.to, Some(1));
    }
}
