use super::fx::hash;
use ratatui::{Frame, style::Style};

pub fn draw(f: &mut Frame, frame: u32) {
    let (area, palette) = (f.area(), super::theme::get());
    if area.is_empty() {
        return;
    }
    let (w, h) = (area.width.max(1) as u32, area.height.max(1) as u32);
    for i in 0..60u32 {
        let speed = 1 + hash(i * 11) % 4;
        let x = area.x + (hash(i) % w) as u16;
        let y = area.y + ((hash(i * 5) + frame * speed / 4) % h) as u16;
        let glyph = ["○", "●", "·"][i as usize % 3];
        let color = palette.backdrop[i as usize % 3];
        f.buffer_mut()[(x, y)].set_symbol(glyph).set_style(Style::default().fg(color));
    }
}
