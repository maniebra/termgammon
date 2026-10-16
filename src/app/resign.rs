use super::{Fx, Game, Phase};
use crate::domain::ai;

impl Game {
    pub fn resign(&mut self, mult: u32) {
        if self.phase == Phase::Over {
            return;
        }
        let loser = self.ai.map_or(self.actor(), |a| a.other());
        let winner = loser.other();
        if self.ai == Some(winner) && !ai::accepts_resign(&self.board, winner, mult) {
            self.log.push(format!("{winner:?} declines the resignation"));
            return self.fx(Fx::Toast("DECLINED".into()));
        }
        self.log.push(format!("{loser:?} resigns"));
        self.fx(Fx::Toast("RESIGNED".into()));
        self.finish(winner, self.cube.value * mult);
    }
}
