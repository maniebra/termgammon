use super::{Action, Game, OFF, Phase};
use crate::domain::Player;

impl Game {
    pub fn handle(&mut self, a: Action) {
        match (a, self.phase) {
            _ if self.ai == Some(self.actor()) => {}
            (Action::Roll, Phase::Roll) => {
                self.roll();
                self.advance();
            }
            (Action::Double, Phase::Roll) if self.cube.can_double(self.turn) => self.offer(),
            (Action::Take, Phase::Offer) => {
                self.accept();
                self.advance();
            }
            (Action::Pass, Phase::Offer) => self.decline(),
            (Action::Left, Phase::Move) => self.cursor = (self.cursor + OFF) % (OFF + 1),
            (Action::Right, Phase::Move) => self.cursor = (self.cursor + 1) % (OFF + 1),
            (Action::Cancel, Phase::Move) => self.selected = None,
            (Action::Enter, Phase::Move) => self.enter(),
            (Action::Undo, Phase::Move) => self.undo(),
            (Action::Next, Phase::Move) => self.next(),
            (Action::Click(i), Phase::Move) => {
                self.cursor = i;
                self.enter();
            }
            (Action::Enter, Phase::Over) => self.reset(),
            _ => {}
        }
    }

    pub fn set_ai(&mut self, ai: Option<Player>) {
        self.ai = ai;
        self.selected = None;
        self.advance();
    }

    fn enter(&mut self) {
        let at = (self.cursor < OFF).then_some(self.cursor);
        let legal = self.legal();
        let Some(from) = self.source() else {
            match legal.iter().any(|m| m.from == at) {
                true => self.selected = Some(at),
                false => self.msg = "No move from there".into(),
            }
            return;
        };
        self.selected = None;
        match legal.into_iter().find(|m| m.from == from && m.to == at) {
            Some(m) => {
                self.msg.clear();
                self.snapshot();
                self.play(m);
                self.advance();
            }
            None => self.msg = "Illegal move".into(),
        }
    }
}
