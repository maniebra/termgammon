use crate::app::Game;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub fn art(g: &Game) -> Vec<Line<'static>> {
    let faces: Vec<(u8, bool)> = g
        .rolled
        .iter()
        .enumerate()
        .map(|(i, &d)| match g.anim {
            0 => (d, g.dice.contains(&d)),
            a => ((d as usize + a as usize * (i + 2)) as u8 % 6 + 1, true),
        })
        .collect();
    let extra = match g.dice.len() {
        n if g.anim == 0 && g.rolled.len() == 2 && g.rolled[0] == g.rolled[1] => format!(" ×{n}"),
        _ => String::new(),
    };
    (0..5)
        .map(|row| {
            let mut spans: Vec<Span> = faces.iter().map(|&(d, live)| face(d, live, row)).collect();
            if row == 2 {
                spans.push(Span::raw(extra.clone()));
            }
            Line::from(spans)
        })
        .collect()
}

fn face(d: u8, live: bool, row: usize) -> Span<'static> {
    let style = Style::default().fg(if live { Color::White } else { Color::DarkGray });
    let text = match row {
        0 => "╭───────╮".to_string(),
        4 => "╰───────╯".to_string(),
        r => {
            let cells: Vec<&str> = (0..3).map(|c| if pip(d, r - 1, c) { "●" } else { " " }).collect();
            format!("│ {} │", cells.join(" "))
        }
    };
    Span::styled(text + " ", style)
}

fn pip(d: u8, r: usize, c: usize) -> bool {
    let corner_main = (r, c) == (0, 0) || (r, c) == (2, 2);
    let corner_anti = (r, c) == (0, 2) || (r, c) == (2, 0);
    let center = (r, c) == (1, 1);
    let sides = r == 1 && c != 1;
    match d {
        1 => center,
        2 => corner_main,
        3 => corner_main || center,
        4 => corner_main || corner_anti,
        5 => corner_main || corner_anti || center,
        _ => corner_main || corner_anti || sides,
    }
}
