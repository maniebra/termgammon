use super::Palette;
use ratatui::style::Color::*;

pub fn classic() -> Palette {
    Palette {
        white: White,
        black: LightRed,
        point: [Rgb(120, 80, 50), Rgb(70, 50, 35)],
        select: [Rgb(255, 215, 0), Rgb(255, 130, 0)],
        target: [Rgb(30, 140, 60), Rgb(100, 235, 120)],
        last: Rgb(40, 80, 130),
        accent: Yellow,
        dim: DarkGray,
        border: Reset,
        backdrop: [Rgb(70, 70, 80), Rgb(110, 40, 40), Rgb(60, 60, 60)],
    }
}

pub fn ocean() -> Palette {
    Palette {
        white: Rgb(230, 245, 255),
        black: Rgb(255, 170, 60),
        point: [Rgb(20, 70, 110), Rgb(10, 40, 75)],
        select: [Rgb(120, 220, 255), Rgb(40, 160, 255)],
        target: [Rgb(0, 160, 150), Rgb(80, 255, 220)],
        last: Rgb(90, 60, 150),
        accent: LightCyan,
        dim: Rgb(80, 110, 140),
        border: Rgb(60, 140, 200),
        backdrop: [Rgb(30, 70, 110), Rgb(120, 80, 30), Rgb(40, 60, 90)],
    }
}

pub fn neon() -> Palette {
    Palette {
        white: Rgb(0, 255, 210),
        black: Rgb(255, 40, 180),
        point: [Rgb(45, 0, 70), Rgb(15, 0, 35)],
        select: [Rgb(255, 255, 0), Rgb(255, 120, 255)],
        target: [Rgb(0, 200, 255), Rgb(120, 255, 255)],
        last: Rgb(90, 0, 120),
        accent: Rgb(255, 80, 255),
        dim: Rgb(120, 70, 160),
        border: Rgb(180, 0, 255),
        backdrop: [Rgb(0, 90, 80), Rgb(100, 0, 70), Rgb(50, 0, 80)],
    }
}
