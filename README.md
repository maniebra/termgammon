# termgammon

Backgammon in the terminal, built with [ratatui](https://ratatui.rs).

## Features

- Full rules: hitting, the bar, bearing off, and the rule that you must use as many dice as possible
- Play against an AI (easy, medium, hard) or against a friend on the same keyboard
- Doubling cube, gammons and backgammons, and matches to 1, 3, 5, 7 or 9 points
- Animated dice, highlighted last move, pip-count race gauges and a move log
- Undo within a turn, keyboard and mouse controls
- Themes: classic, ocean, neon, forest, mono and basic (16 colors, for terminals without truecolor)

## Install

Prebuilt binaries for Linux, macOS and Windows are attached to each release.

From source (needs a Rust toolchain):

```sh
make install     # cargo install --path . --locked
make uninstall
```

## Usage

```sh
termgammon [--level easy|medium|hard] [--side black|white] [--length N] [--theme NAME]
```

Theme, level, AI side and match length are saved on quit to
`$XDG_CONFIG_HOME/termgammon/settings` (or `~/.config/termgammon/settings`).
Command-line flags override saved values.

## Controls

| Key | Action |
| --- | --- |
| arrows | move the cursor (↑/↓ switch rows) |
| tab | jump to the next playable spot |
| enter / click | pick a checker, then a green spot |
| u | undo a move |
| r | roll |
| d | offer a double |
| y / p | take / pass a double |
| esc / q | menu |

## Development

```sh
make test
make build
```

The code follows a small DDD layout:

- `src/domain` – board, rules, cube, scoring and AI
- `src/app` – game flow, menus and input handling
- `src/infra` – terminal, keys, mouse, settings and rendering
- `src/shared` – dice randomness

## License

[MIT](LICENSE)
