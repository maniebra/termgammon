use super::{Action, Game, OFF, Phase};
use crate::domain::Player;

impl Game {
    pub fn handle(&mut self, a: Action) {
        match (a, self.phase) {
            (Action::CycleLevel, _) => self.level = self.level.next(),
            (Action::CycleMode, _) => self.cycle_mode(),
            (Action::NewGame, _) => self.reset(),
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
            _ => {}
        }
    }

    fn cycle_mode(&mut self) {
        self.ai = match self.ai {
            Some(Player::Black) => Some(Player::White),
            Some(Player::White) => None,
            None => Some(Player::Black),
        };
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
                self.play(m);
                self.advance();
            }
            None => self.msg = "Illegal move".into(),
        }
    }
}
