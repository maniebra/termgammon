use super::noise::{hash, put};
use ratatui::{
    Frame,
    style::{Color, Style},
};

const LIFE: u32 = 120;
const GLYPHS: [&str; 6] = ["▪", "▴", "◆", "●", "✦", "❖"];
const COLORS: [Color; 6] =
    [Color::Yellow, Color::LightRed, Color::LightMagenta, Color::LightCyan, Color::LightGreen, Color::White];

pub fn draw(f: &mut Frame, age: u32) {
    if age >= LIFE {
        return;
    }
    let area = f.area();
    for i in 0..120u32 {
        let speed = 0.25 + (hash(i * 7) % 100) as f32 / 200.0;
        let y = area.y as f32 - (hash(i * 3) % area.height.max(1) as u32) as f32 + age as f32 * speed;
        let sway = (age as f32 * 0.2 + i as f32).sin() * 1.5;
        let x = area.x as f32 + (hash(i) % area.width.max(1) as u32) as f32 + sway;
        let style = Style::default().fg(COLORS[i as usize % COLORS.len()]);
        put(f.buffer_mut(), area, x, y, GLYPHS[hash(i + age / 6) as usize % GLYPHS.len()], style);
    }
}
