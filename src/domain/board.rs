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
    pub fn count(&self, p: Player, i: usize) -> i8 {
        (self.points[i] * p.sign()).max(0)
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
