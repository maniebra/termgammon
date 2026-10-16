#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Classic,
    Ocean,
    Neon,
    Forest,
    Mono,
    Basic,
}

pub const THEMES: [Theme; 6] = [Theme::Classic, Theme::Ocean, Theme::Neon, Theme::Forest, Theme::Mono, Theme::Basic];

impl Theme {
    pub fn next(self) -> Self {
        THEMES[(THEMES.iter().position(|&t| t == self).unwrap() + 1) % THEMES.len()]
    }
}
