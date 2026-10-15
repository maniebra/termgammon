use super::Game;
use crate::domain::Move;

impl Game {
    pub(super) fn play(&mut self, m: Move) {
        self.board.apply(self.turn, m);
        let i = self.dice.iter().position(|&d| d == m.die).unwrap();
        self.dice.remove(i);
    }

    pub(super) fn roll(&mut self) {
        let (a, b) = (self.rng.die(), self.rng.die());
        self.dice = if a == b { vec![a; 4] } else { vec![a, b] };
    }

    pub(super) fn advance(&mut self) {
        if let Some(p) = self.board.winner() {
            self.msg = format!("{p:?} wins! Press q to quit");
            return;
        }
        while self.legal().is_empty() {
            if !self.dice.is_empty() {
                self.msg = format!("{:?} rolled {:?}, no moves", self.turn, self.dice);
            }
            self.turn = self.turn.other();
            self.roll();
        }
    }
}
