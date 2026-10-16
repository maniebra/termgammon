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
            (Action::Double, Phase::Roll) if self.may_double() => self.offer(),
            (Action::Take, Phase::Offer) => {
                self.accept();
                self.advance();
            }
            (Action::Pass, Phase::Offer) => self.decline(),
            (Action::Left, Phase::Move) => self.step(12, false),
            (Action::Right, Phase::Move) => self.step(1, false),
            (Action::Up | Action::Down, Phase::Move) => self.step(0, true),
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
        let movable = legal.iter().any(|m| m.from == at);
        let chosen = self.source().and_then(|from| legal.into_iter().find(|m| m.from == from && m.to == at));
        self.msg.clear();
        match (chosen, self.source()) {
            (Some(m), _) => {
                self.selected = None;
                self.snapshot();
                self.play(m);
                self.advance();
            }
            (None, Some(from)) if from == at => self.selected = None,
            (None, _) if movable && self.board.bar[self.turn.idx()] == 0 => {
                self.selected = Some(at);
                self.next();
            }
            (None, Some(_)) => self.msg = "Can't move there, pick a green spot".into(),
            (None, None) => self.msg = "No legal move from that point".into(),
        }
    }
}
