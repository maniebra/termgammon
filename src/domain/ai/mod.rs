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

    #[test]
    fn plays_full_game() {
        let (mut b, mut p, mut rng) = (Board::default(), Player::White, Rng::new());
        for _ in 0..1000 {
            let (x, y) = (rng.die(), rng.die());
            let mut dice = if x == y { vec![x; 4] } else { vec![x, y] };
            let level = if p == Player::White { Level::Hard } else { Level::Medium };
            while let Some(m) = best(&b, p, &dice, level, &mut rng) {
                b.apply(p, m);
                dice = crate::domain::without(&dice, m.die);
            }
            if b.winner().is_some() {
                return;
            }
            p = p.other();
        }
        panic!("game never ended");
    }
}
