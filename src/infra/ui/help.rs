use ratatui::{
    style::{Color, Style},
    text::Line,
};

pub fn banner() -> Vec<Line<'static>> {
    let style = Style::default().fg(Color::LightRed);
    ["●  ○  ●  ○  ●  ○", "  t e r m g a m m o n", "○  ●  ○  ●  ○  ●"].map(|s| Line::styled(s, style)).to_vec()
}

pub fn rules() -> Vec<Line<'static>> {
    [
        "Move all 15 checkers home, then bear them off.",
        "White moves 24 → 1, Black moves 1 → 24.",
        "A single checker (blot) can be hit and sent to the bar.",
        "Checkers on the bar must re-enter before anything else.",
        "",
        "r roll   d double   y take   p pass",
        "←/→ move cursor   enter pick / drop   esc menu",
        "",
        "Gammon (loser bore off none) scores 2×, backgammon 3×.",
    ]
    .map(Line::raw)
    .to_vec()
}
