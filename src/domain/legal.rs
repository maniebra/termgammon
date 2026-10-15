use super::{Board, Move, Player, moves::without};

impl Board {
    pub fn legal_moves(&self, p: Player, dice: &[u8]) -> Vec<Move> {
        let scored: Vec<(Move, usize)> =
            self.single_moves(p, dice).into_iter().map(|m| (m, 1 + self.chain_after(p, dice, m))).collect();
        let best = scored.iter().map(|s| s.1).max().unwrap_or(0);
        let mut out: Vec<Move> = scored.into_iter().filter(|s| s.1 == best).map(|s| s.0).collect();
        if best == 1 {
            let high = out.iter().map(|m| m.die).max();
            out.retain(|m| Some(m.die) == high);
        }
        out
    }

    fn chain_after(&self, p: Player, dice: &[u8], m: Move) -> usize {
        let mut next = self.clone();
        next.apply(p, m);
        next.longest_chain(p, &without(dice, m.die))
    }

    fn longest_chain(&self, p: Player, dice: &[u8]) -> usize {
        let mut best = 0;
        for m in self.single_moves(p, dice) {
            best = best.max(1 + self.chain_after(p, dice, m));
            if best == dice.len() {
                break;
            }
        }
        best
    }
}
