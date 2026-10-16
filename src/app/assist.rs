use super::{Game, OFF};
use crate::domain::ai::{self, Level};

impl Game {
    pub(super) fn snapshot(&mut self) {
        self.undo.push((self.board.clone(), self.dice.clone(), self.log.last().cloned()));
    }

    pub(super) fn undo(&mut self) {
        let Some((board, dice, line)) = self.undo.pop() else { return };
        self.board = board;
        self.dice = dice;
        self.last.pop();
        self.selected = None;
        if let (Some(last), Some(line)) = (self.log.last_mut(), line) {
            *last = line;
        }
    }

    pub(super) fn hint(&mut self) {
        let Some(m) = ai::best(&self.board, self.turn, &self.dice, Level::Hard, &mut self.rng) else { return };
        self.selected = Some(m.from);
        self.cursor = m.to.unwrap_or(OFF);
        let at = |p: Option<usize>, edge: &str| p.map_or(edge.into(), |i| (i + 1).to_string());
        self.msg = format!("Hint: {}/{}, press enter to play it", at(m.from, "bar"), at(m.to, "off"));
    }
}
