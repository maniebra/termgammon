use super::style;
use crate::app::Game;
use ratatui::{
    style::Color,
    text::{Line, Span},
};

pub fn labels(ix: &[usize]) -> Line<'static> {
    Line::from(row(ix, |i| Span::raw(format!("{:>3} ", i + 1))))
}

pub fn cells<'a>(g: &Game, ix: &[usize], targets: &[Option<usize>]) -> Line<'a> {
    Line::from(row(ix, |i| {
        let (text, fg) = match g.board.points[i] {
            0 => ("  . ".to_string(), Color::DarkGray),
            v if v > 0 => (format!(" W{:<2}", v), Color::White),
            v => (format!(" B{:<2}", -v), Color::Red),
        };
        Span::styled(text, style::of(g, i, targets, fg))
    }))
}

fn row<'a>(ix: &[usize], cell: impl Fn(usize) -> Span<'a>) -> Vec<Span<'a>> {
    let mut spans: Vec<Span> = ix.iter().map(|&i| cell(i)).collect();
    spans.insert(6, Span::raw(" | "));
    spans
}
