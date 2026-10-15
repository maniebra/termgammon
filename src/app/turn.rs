use super::Game;
use crate::domain::{Move, ai, without};

impl Game {
    pub(super) fn play(&mut self, m: Move) {
        self.board.apply(self.turn, m);
        self.dice = without(&self.dice, m.die);
        self.note(&format!(" {}-{}", pos(m.from, "bar"), pos(m.to, "off")));
    }

    pub(super) fn roll(&mut self) {
        let (a, b) = (self.rng.die(), self.rng.die());
        self.dice = if a == b { vec![a; 4] } else { vec![a, b] };
        self.log.push(format!("{:?} rolled {:?}:", self.turn, self.dice));
    }

    pub(super) fn advance(&mut self) {
        while self.board.winner().is_none() {
            if self.legal().is_empty() {
                if !self.dice.is_empty() {
                    self.note(" no moves");
                }
                self.turn = self.turn.other();
                self.roll();
            } else if self.ai == Some(self.turn) {
                let m = ai::best(&self.board, self.turn, &self.dice, self.level, &mut self.rng).unwrap();
                self.play(m);
            } else {
                return;
            }
        }
        self.log.push(format!("{:?} wins! Press n for a new game", self.board.winner().unwrap()));
    }

    fn note(&mut self, s: &str) {
        if let Some(last) = self.log.last_mut() {
            last.push_str(s);
        }
    }
}

fn pos(p: Option<usize>, edge: &str) -> String {
    p.map_or(edge.into(), |i| (i + 1).to_string())
}
