use super::Screen;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    VsAi,
    Hotseat,
    Resume,
    NewGame,
    Settings,
    Help,
    MainMenu,
    Quit,
    Level,
    Side,
    Back,
}

pub fn items(s: Screen) -> &'static [Item] {
    use Item::*;
    match s {
        Screen::Main => &[VsAi, Hotseat, Settings, Help, Quit],
        Screen::Pause => &[Resume, NewGame, Settings, Help, MainMenu, Quit],
        Screen::Settings => &[Level, Side, Back],
        Screen::Help => &[Back],
        Screen::Playing => &[],
    }
}
