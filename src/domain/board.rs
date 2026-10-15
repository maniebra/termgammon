use super::{Move, Player};

#[derive(Clone)]
pub struct Board {
    pub points: [i8; 24],
    pub bar: [u8; 2],
    pub off: [u8; 2],
}

impl Default for Board {
    fn default() -> Self {
        let mut points = [0i8; 24];
        for (i, n) in [(23, 2), (12, 5), (7, 3), (5, 5)] {
            points[i] = n;
            points[23 - i] = -n;
        }
        Self { points, bar: [0; 2], off: [0; 2] }
    }
}

impl Board {
    fn count(&self, p: Player, i: usize) -> i8 {
        (self.points[i] * p.sign()).max(0)
    }

    fn all_home(&self, p: Player) -> bool {
        self.bar[p.idx()] == 0 && (0..24).filter(|&i| !in_home(p, i)).all(|i| self.count(p, i) == 0)
    }

    // ponytail: no forced max-dice-usage rule; any legal single-die move is allowed
    pub fn legal_moves(&self, p: Player, dice: &[u8]) -> Vec<Move> {
        let mut dice = dice.to_vec();
        dice.sort();
        dice.dedup();
        let sources: Vec<Option<usize>> = if self.bar[p.idx()] > 0 {
            vec![None]
        } else {
            (0..24).filter(|&i| self.count(p, i) > 0).map(Some).collect()
        };
        sources
            .iter()
            .flat_map(|&from| dice.iter().filter_map(move |&die| self.try_move(p, from, die)))
            .collect()
    }

    fn try_move(&self, p: Player, from: Option<usize>, die: u8) -> Option<Move> {
        let white = p == Player::White;
        let start = from.map_or(if white { 24 } else { -1 }, |i| i as i32);
        let t = start + if white { -(die as i32) } else { die as i32 };
        if (0..24).contains(&t) {
            let to = t as usize;
            return (self.points[to] * p.sign() >= -1).then_some(Move { from, to: Some(to), die });
        }
        let f = from?;
        let exact = t == -1 || t == 24;
        let farther = (0..24).any(|i| (if white { i > f } else { i < f }) && self.count(p, i) > 0);
        (self.all_home(p) && (exact || !farther)).then_some(Move { from, to: None, die })
    }

    pub fn apply(&mut self, p: Player, m: Move) {
        let s = p.sign();
        match m.from {
            None => self.bar[p.idx()] -= 1,
            Some(i) => self.points[i] -= s,
        }
        match m.to {
            None => self.off[p.idx()] += 1,
            Some(t) => {
                if self.points[t] == -s {
                    self.points[t] = 0;
                    self.bar[p.other().idx()] += 1;
                }
                self.points[t] += s;
            }
        }
    }

    pub fn winner(&self) -> Option<Player> {
        [Player::White, Player::Black].into_iter().find(|p| self.off[p.idx()] == 15)
    }
}

fn in_home(p: Player, i: usize) -> bool {
    if p == Player::White { i < 6 } else { i >= 18 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules() {
        let mut b = Board::default();
        assert!(b.legal_moves(Player::White, &[6]).contains(&Move { from: Some(23), to: Some(17), die: 6 }));
        assert!(!b.legal_moves(Player::White, &[5]).iter().any(|m| m.from == Some(23)));
        b.points = [0; 24];
        b.points[3] = -1;
        b.points[5] = 1;
        b.apply(Player::White, Move { from: Some(5), to: Some(3), die: 2 });
        assert_eq!((b.points[3], b.bar[1]), (1, 1));
        b.points[2] = 2;
        assert_eq!(b.legal_moves(Player::Black, &[3, 6]), vec![Move { from: None, to: Some(5), die: 6 }]);
        assert_eq!(b.legal_moves(Player::White, &[6])[0].to, None);
    }
}
