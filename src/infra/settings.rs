use super::names;
use crate::app::App;
use std::{fs, path::PathBuf};

fn path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("termgammon").join("settings"))
}

pub fn load(a: &mut App) {
    let text = path().and_then(|p| fs::read_to_string(p).ok()).unwrap_or_default();
    apply(a, text.lines().filter_map(|l| l.split_once('=')));
}

pub fn args(a: &mut App, args: &[String]) {
    apply(a, args.chunks(2).filter_map(|c| Some((c[0].strip_prefix("--")?, c.get(1)?.as_str()))));
}

fn apply<'a>(a: &mut App, pairs: impl Iterator<Item = (&'a str, &'a str)>) {
    for (key, value) in pairs {
        match key {
            "level" => a.game.level = names::level(value).unwrap_or(a.game.level),
            "side" => a.side = names::side(value).unwrap_or(a.side),
            "animations" => a.game.motion = value != "off",
            "theme" => a.theme = names::theme(value).unwrap_or(a.theme),
            "seed" => value.parse().into_iter().for_each(|s| a.game.seed(s)),
            "length" => a.game.target = value.parse().unwrap_or(a.game.target).clamp(1, 25),
            _ => {}
        }
    }
}

pub fn save(a: &App) {
    let Some(p) = path() else { return };
    let (level, side, theme) = (names::lower(a.game.level), names::lower(a.side), names::lower(a.theme));
    let text = format!(
        "level={level}\nside={side}\nlength={}\ntheme={theme}\nanimations={}\n",
        a.game.target,
        if a.game.motion { "on" } else { "off" }
    );
    let _ = p.parent().map(fs::create_dir_all);
    let _ = fs::write(p, text);
}
