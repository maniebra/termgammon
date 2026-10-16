use super::{Game, Phase};
use crate::domain::ai;

const AI_DELAY: u8 = 18;

impl Game {
    pub fn tick(&mut self) {
        if self.anim > 0 {
            self.anim -= 1;
            return;
        }
        if self.wait > 0 {
            self.wait -= 1;
            return;
        }
        if !self.thinking() {
            return;
        }
        match self.phase {
            Phase::Roll if self.may_double() && ai::wants_double(&self.board, self.turn) => self.offer(),
            Phase::Roll => self.roll(),
            _ => {
                let m = ai::best(&self.board, self.turn, &self.dice, self.level, &mut self.rng).unwrap();
                self.play(m);
            }
        }
        self.wait = AI_DELAY;
        self.advance();
    }

    pub fn thinking(&self) -> bool {
        self.ai == Some(self.turn) && matches!(self.phase, Phase::Roll | Phase::Move)
    }
}
