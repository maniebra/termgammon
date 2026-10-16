use crate::domain::{Board, Player};

// ponytail: pip-based thresholds, real cube equity needs rollouts
pub fn wants_double(b: &Board, p: Player) -> bool {
    b.eval(p) > 20
}

pub fn accepts_resign(b: &Board, p: Player, offered: u32) -> bool {
    let risk = if b.win_chance(p) > 0.85 { b.multiplier(p) } else { 1 };
    offered >= risk
}

pub fn takes(b: &Board, p: Player) -> bool {
    b.eval(p) > -25
}
