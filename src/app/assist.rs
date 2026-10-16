use super::Game;

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
}
