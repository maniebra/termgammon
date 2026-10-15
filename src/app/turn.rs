use super::{Game, Phase};
use crate::domain::{Move, without};

impl Game {
    pub(super) fn play(&mut self, m: Move) {
        self.board.apply(self.turn, m);
        self.dice = without(&self.dice, m.die);
        self.note(&format!(" {}-{}", pos(m.from, "bar"), pos(m.to, "off")));
    }

    pub(super) fn roll(&mut self) {
        let (a, b) = (self.rng.die(), self.rng.die());
        self.dice = if a == b { vec![a; 4] } else { vec![a, b] };
        self.phase = Phase::Move;
        self.log.push(format!("{:?} rolled {:?}:", self.turn, self.dice));
    }

    pub(super) fn advance(&mut self) {
        loop {
            match self.phase {
                Phase::Move if self.board.winner().is_some() => {
                    let w = self.board.winner().unwrap();
                    return self.finish(w, self.cube.value * self.board.multiplier(w));
                }
                Phase::Move if self.legal().is_empty() => self.end_turn(),
                _ => return,
            }
        }
    }

    fn end_turn(&mut self) {
        if !self.dice.is_empty() {
            self.note(" no moves");
        }
        self.dice.clear();
        self.selected = None;
        self.turn = self.turn.other();
        self.phase = Phase::Roll;
    }

    pub(super) fn note(&mut self, s: &str) {
        if let Some(last) = self.log.last_mut() {
            last.push_str(s);
        }
    }
}

fn pos(p: Option<usize>, edge: &str) -> String {
    p.map_or(edge.into(), |i| (i + 1).to_string())
}
