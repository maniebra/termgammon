use super::Game;
use crate::domain::{Move, Player, ai};

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
        while self.board.winner().is_none() {
            if self.legal().is_empty() {
                if !self.dice.is_empty() {
                    self.msg = format!("{:?} rolled {:?}, no moves", self.turn, self.dice);
                }
                self.turn = self.turn.other();
                self.roll();
                if self.turn == Player::Black {
                    self.msg = format!("Black rolled {:?}:", self.dice);
                }
            } else if self.turn == Player::Black {
                let m = ai::best(&self.board, self.turn, &self.dice, ai::Level::Medium, &mut self.rng).unwrap();
                self.msg += &format!(" {}-{}", pos(m.from, "bar"), pos(m.to, "off"));
                self.play(m);
            } else {
                return;
            }
        }
        self.msg = format!("{:?} wins! Press q to quit", self.board.winner().unwrap());
    }
}

fn pos(p: Option<usize>, edge: &str) -> String {
    p.map_or(edge.into(), |i| (i + 1).to_string())
}
