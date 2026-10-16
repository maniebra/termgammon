use ratatui::{buffer::Buffer, layout::Rect, style::Style};

pub fn hash(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^ (x >> 16)
}

pub fn put(buf: &mut Buffer, area: Rect, x: f32, y: f32, symbol: &str, style: Style) {
    let (x, y) = (x.round() as i32, y.round() as i32);
    let inside =
        x >= area.left() as i32 && x < area.right() as i32 && y >= area.top() as i32 && y < area.bottom() as i32;
    if inside {
        buf[(x as u16, y as u16)].set_symbol(symbol).set_style(style);
    }
}
