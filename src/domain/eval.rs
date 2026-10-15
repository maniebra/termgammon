use super::{Board, Player};

impl Board {
    pub fn pips(&self, p: Player) -> i32 {
        let dist = |i: usize| if p == Player::White { i + 1 } else { 24 - i } as i32;
        let on_board: i32 = (0..24).map(|i| self.count(p, i) as i32 * dist(i)).sum();
        on_board + 25 * self.bar[p.idx()] as i32
    }

    fn blots(&self, p: Player) -> i32 {
        (0..24).filter(|&i| self.count(p, i) == 1).count() as i32
    }

    pub fn eval(&self, p: Player) -> i32 {
        self.pips(p.other()) - self.pips(p) - 4 * self.blots(p)
    }
}
