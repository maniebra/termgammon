use super::{Game, OFF, Phase};
use crate::domain::Player;

impl Game {
    fn cell(&self, i: usize) -> (usize, usize) {
        match i {
            OFF => (self.off_row(), 12),
            12.. => (0, i - 12),
            _ => (1, 11 - i),
        }
    }

    fn at(r: usize, c: usize) -> usize {
        match (r, c) {
            (_, 12) => OFF,
            (0, c) => 12 + c,
            (_, c) => 11 - c,
        }
    }

    fn off_row(&self) -> usize {
        if self.turn == Player::White { 1 } else { 0 }
    }

    pub(super) fn step(&mut self, dc: usize, flip: bool) {
        let (r, c) = self.cell(self.cursor);
        let r = if flip && c != 12 { 1 - r } else { r };
        self.cursor = Self::at(r, (c + dc) % 13);
    }

    pub(super) fn next(&mut self) {
        let mut spots: Vec<(usize, usize)> = self.spots().into_iter().map(|i| self.cell(i)).collect();
        spots.sort();
        let here = self.cell(self.cursor);
        if let Some(&(r, c)) = spots.iter().find(|&&s| s > here).or(spots.first()) {
            self.cursor = Self::at(r, c);
        }
    }

    pub(super) fn aim(&mut self) {
        let human = self.phase == Phase::Move && !self.thinking() && self.selected.is_none();
        if human && !self.spots().contains(&self.cursor) {
            self.next();
        }
    }

    fn spots(&self) -> Vec<usize> {
        match self.source() {
            Some(_) => self.targets().iter().map(|t| t.unwrap_or(OFF)).collect(),
            None => self.legal().iter().filter_map(|m| m.from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_like_the_screen() {
        let mut g = Game::new();
        g.turn = Player::White;
        g.cursor = 5;
        g.step(1, false);
        assert_eq!(g.cursor, 4);
        g.step(0, true);
        assert_eq!(g.cursor, 19);
        g.cursor = 0;
        g.step(1, false);
        assert_eq!(g.cursor, OFF);
        g.step(1, false);
        assert_eq!(g.cursor, 11);
    }
}
