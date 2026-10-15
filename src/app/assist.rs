use super::{Game, OFF};

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

    pub(super) fn next(&mut self) {
        let spots: Vec<usize> = match self.source() {
            Some(_) => self.targets().iter().map(|t| t.unwrap_or(OFF)).collect(),
            None => self.legal().iter().filter_map(|m| m.from).collect(),
        };
        let after = |i: &&usize| **i > self.cursor;
        if let Some(&i) = spots.iter().filter(after).min().or(spots.iter().min()) {
            self.cursor = i;
        }
    }
}
