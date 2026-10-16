use super::Game;
use crate::domain::is_crawford;

impl Game {
    pub fn may_double(&self) -> bool {
        !self.crawford && self.cube.can_double(self.turn)
    }

    pub(super) fn check_crawford(&mut self) {
        self.crawford = is_crawford(self.score, self.target, self.crawford_played);
        if self.crawford {
            self.crawford_played = true;
            self.log.push("Crawford game: no doubling".into());
        }
    }
}
