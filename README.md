<div align="center">

# 🎲 termgammon

### The oldest game in the world. In the coolest place on your computer.

**Backgammon, beautifully rebuilt for the terminal.**
No browser. No ads. No account. Just you, the dice, and a worthy opponent.

![MIT](https://img.shields.io/badge/license-MIT-blue) ![Rust](https://img.shields.io/badge/built%20with-Rust-orange) ![Platforms](https://img.shields.io/badge/linux%20%7C%20macOS%20%7C%20windows-supported-brightgreen)

</div>

```
┌ termgammon ────────────────────────────────────┐┌ game ─────────────────────┐
│13 14 15 16 17 18  ┃ 19 20 21 22 23 24  ┃ off   ││Turn   ● White             │
│ ●           ●     ┃  ●              ●  ┃  0    ││╭───────╮ ╭───────╮        │
│ ●           ●     ┃  ●              ●  ┃       │││ ●   ● │ │ ●     │        │
│ ●           ●     ┃  ●                 ┃       │││ ●   ● │ │   ●   │        │
│ ●                 ┃  ●                 ┃       │││ ●   ● │ │     ● │        │
│ ●                 ┃  ●                 ┃       ││╰───────╯ ╰───────╯        │
│                  BAR                           ││Cube   2 (White)           │
│ ●                 ┃  ●                 ┃       ││Score  White 2  Black 1    │
│ ●                 ┃  ●                 ┃       ││                           │
│ ●           ●     ┃  ●                 ┃       ││Move to a green spot       │
│ ●           ●     ┃  ●              ●  ┃       │└───────────────────────────┘
│ ●           ●     ┃  ●              ●  ┃  0    │
│12 11 10  9  8  7  ┃  6  5  4  3  2  1  ┃ off   │
└────────────────────────────────────────────────┘
```

## Why you'll love it

🔥 **It looks incredible.** Checkers fly across the board in smooth arcs. Hits explode in sparks. Dice tumble. Win a match and the whole screen rains confetti. Yes, in a terminal.

🧠 **An AI that fights back.** Warm up on Easy, get humbled on Hard. Hard looks ahead through every way to play your roll, and it knows when to double you.

🎯 **The real game, all of it.** Hitting, the bar, bearing off, the doubling cube, gammons, backgammons, and full matches to 1, 3, 5, 7 or 9 points.

🎨 **Make it yours.** Six themes: Classic, Ocean, Neon, Forest, Mono and Basic. One of them is going to match your setup perfectly.

🖱️ **Plays the way you think.** Click a checker, click where it goes. Or fly around with the keyboard. Legal moves glow green, and a wrong move is one keypress to undo.

👥 **Bring a friend.** Hotseat mode for two players on one keyboard. Settle it once and for all.

⚡ **Instant.** Starts in a blink, runs anywhere, fits on small screens too.

## Get it in 10 seconds

**Download** a ready-to-run build for Linux, macOS or Windows from the [Releases](../../releases) page.

**Or build it yourself:**

```sh
make install
termgammon
```

## How to play

Pick a checker, pick a green spot. That's it. Everything else you'll pick up as you go.

| | |
| --- | --- |
| **click** or **enter** | pick up / drop a checker |
| **arrows** · **tab** | move around · jump to the next playable checker |
| **r** · **d** | roll · double the stakes |
| **y** · **p** | take · pass a double |
| **u** | undo |
| **esc** | menu |

New to backgammon? Open **How to play** from the main menu for the rules in thirty seconds.

## For tinkerers

```sh
termgammon --level hard --side white --length 5 --theme neon
```

Your choices are remembered between sessions. Contributions are welcome: `make test` and you're off.

<div align="center">

**Roll the dice. ⚀ ⚁ ⚂ ⚃ ⚄ ⚅**

MIT licensed · made with ♥ and Rust

</div>
