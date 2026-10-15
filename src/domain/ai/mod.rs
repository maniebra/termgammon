mod cube;
mod level;
mod search;

pub use cube::{takes, wants_double};
pub use level::Level;

use super::{Board, Move, Player};
use crate::shared::Rng;

pub fn best(b: &Board, p: Player, dice: &[u8], level: Level, rng: &mut Rng) -> Option<Move> {
    match level {
        Level::Easy => {
            let moves = b.legal_moves(p, dice);
            (!moves.is_empty()).then(|| moves[rng.below(moves.len())])
        }
        Level::Medium => search::pick(b, p, dice, 1),
        Level::Hard => search::pick(b, p, dice, 4),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits_blot() {
        let mut b = Board::default();
        b.points[1] = 1;
        b.points[5] = 4;
        for level in [Level::Medium, Level::Hard] {
            let m = best(&b, Player::Black, &[1], level, &mut Rng::new()).unwrap();
            assert_eq!(m.to, Some(1));
        }
    }
}
