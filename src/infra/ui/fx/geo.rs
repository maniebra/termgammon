use crate::app::Game;
use crate::domain::Player;
use ratatui::layout::Rect;

pub fn source(g: &Game, from: Option<usize>, p: Player, board: Rect) -> (f32, f32) {
    match from {
        None => at(board, if p == Player::White { 9 } else { 30 }, 6),
        Some(i) => point(board, i, g.board.count(p, i) as usize),
    }
}

pub fn dest(g: &Game, to: Option<usize>, p: Player, board: Rect) -> (f32, f32) {
    match to {
        None => at(board, 43, if p == Player::White { 11 } else { 1 }),
        Some(i) => point(board, i, (g.board.count(p, i) as usize).saturating_sub(1)),
    }
}

pub fn point(board: Rect, i: usize, k: usize) -> (f32, f32) {
    let (c, y) = if i >= 12 { (i - 12, 1 + k.min(4)) } else { (11 - i, 11 - k.min(4)) };
    let x = if c < 6 { c * 3 + 1 } else { 22 + (c - 6) * 3 };
    at(board, x, y)
}

fn at(board: Rect, x: usize, y: usize) -> (f32, f32) {
    ((board.x + 1) as f32 + x as f32, (board.y + 1) as f32 + y as f32)
}
