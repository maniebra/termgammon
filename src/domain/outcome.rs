use super::{Board, Player};

impl Board {
    pub fn multiplier(&self, winner: Player) -> u32 {
        let loser = winner.other();
        let trapped = (0..24).any(|i| winner.in_home(i) && self.count(loser, i) > 0);
        match (self.off[loser.idx()], self.bar[loser.idx()] > 0 || trapped) {
            (1.., _) => 1,
            (0, false) => 2,
            (0, true) => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gammons() {
        let mut b = Board { points: [0; 24], ..Board::default() };
        b.points[20] = -15;
        assert_eq!(b.multiplier(Player::White), 2);
        b.points[3] = -1;
        assert_eq!(b.multiplier(Player::White), 3);
        b.off[1] = 1;
        assert_eq!(b.multiplier(Player::White), 1);
    }
}
