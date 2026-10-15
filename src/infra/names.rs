use crate::domain::{Player, ai::Level};

pub fn level(s: &str) -> Option<Level> {
    match s {
        "easy" => Some(Level::Easy),
        "medium" => Some(Level::Medium),
        "hard" => Some(Level::Hard),
        _ => None,
    }
}

pub fn side(s: &str) -> Option<Player> {
    match s {
        "black" => Some(Player::Black),
        "white" => Some(Player::White),
        _ => None,
    }
}

pub fn lower(v: impl std::fmt::Debug) -> String {
    format!("{v:?}").to_lowercase()
}
