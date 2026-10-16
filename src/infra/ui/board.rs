use super::{
    cell::{self, ROWS},
    style,
};
use crate::app::{Game, OFF};
use crate::domain::Player;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

const SEP: &str = " ┃ ";

pub fn lines(g: &Game) -> Vec<Line<'static>> {
    let t = g.targets();
    let top: Vec<usize> = (12..24).collect();
    let bottom: Vec<usize> = (0..12).rev().collect();
    let mut out = vec![half(&top, |i| cell::label(g, i, &t), tray(g, &t, "off"))];
    for r in 0..ROWS {
        out.push(half(&top, |i| cell::checker(g, i, r, &t), tray(g, &t, &off(g, Player::Black, r))));
    }
    out.push(bar(g));
    for r in (0..ROWS).rev() {
        out.push(half(&bottom, |i| cell::checker(g, i, r, &t), tray(g, &t, &off(g, Player::White, r))));
    }
    out.push(half(&bottom, |i| cell::label(g, i, &t), tray(g, &t, "off")));
    out
}

fn half(ix: &[usize], cell: impl Fn(usize) -> Span<'static>, tray: Span<'static>) -> Line<'static> {
    let mut spans: Vec<Span> = ix.iter().map(|&i| cell(i)).collect();
    spans.insert(6, Span::raw(SEP));
    spans.extend([Span::raw(SEP), tray]);
    Line::from(spans)
}

fn off(g: &Game, p: Player, row: usize) -> String {
    if row == 0 { format!("{:>2}", g.board.off[p.idx()]) } else { String::new() }
}

fn tray(g: &Game, t: &[Option<usize>], text: &str) -> Span<'static> {
    Span::styled(format!("{text:<4}"), style::mark(g, OFF, t, Style::default()))
}

fn bar(g: &Game) -> Line<'static> {
    let side = |p: Player| {
        let n = g.board.bar[p.idx()];
        let text = if n > 0 { format!("{:^18}", format!("● ×{n}")) } else { " ".repeat(18) };
        Span::styled(text, Style::default().fg(style::player(p)))
    };
    let label = Span::styled("BAR", Style::default().fg(style::dim()));
    Line::from(vec![side(Player::White), label, side(Player::Black)])
}
