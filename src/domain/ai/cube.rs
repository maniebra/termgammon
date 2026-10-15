use crate::domain::{Board, Player};

// ponytail: pip-based thresholds, real cube equity needs rollouts
pub fn wants_double(b: &Board, p: Player) -> bool {
    b.eval(p) > 20
}

pub fn takes(b: &Board, p: Player) -> bool {
    b.eval(p) > -25
}
