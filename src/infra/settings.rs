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
            "length" => a.game.target = value.parse().unwrap_or(a.game.target).clamp(1, 25),
            _ => {}
        }
    }
}

pub fn save(a: &App) {
    let Some(p) = path() else { return };
    let text =
        format!("level={}\nside={}\nlength={}\n", names::lower(a.game.level), names::lower(a.side), a.game.target);
    let _ = p.parent().map(fs::create_dir_all);
    let _ = fs::write(p, text);
}
