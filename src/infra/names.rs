use crate::domain::{Player, ai::Level};

pub fn level(s: &str) -> Option<Level> {
    match s {
        "easy" => Some(Level::Easy),
        "medium" => Some(Level::Medium),
        "hard" => Some(Level::Hard),
        _ => None,
    }
}

pub fn ai(s: &str) -> Option<Option<Player>> {
    match s {
        "black" => Some(Some(Player::Black)),
        "white" => Some(Some(Player::White)),
        "none" => Some(None),
        _ => None,
    }
}

pub fn ai_name(p: Option<Player>) -> String {
    p.map_or("none".into(), |p| format!("{p:?}").to_lowercase())
}
