# tmux-session-namegen

A small, fast Rust program that generates random session names for tmux by pairing an adjective with an emoji (e.g., `stellar-🚀`, `golden-🦄`), similar to how [Docker generates random container names](https://github.com/moby/moby/blob/master/internal/namesgenerator/names-generator.go).

## Example Output

```
$ tmux-session-namegen
stellar-🚀

$ tmux-session-namegen
golden-🦄

$ tmux-session-namegen
cosmic-🌍
```

Currently generates from **149 adjectives** and **262 emojis**, giving **39,038 unique combinations**.

## Emoji Selection

Emojis are organized into categories: animals, plants, fruits, vegetables, food, seafood, weather, celestial, objects, vehicles, sports/music, elements, and buildings.

The following types of emojis were intentionally excluded:

- **Half-width emojis** - render inconsistently across terminals
- **Emojis with skin tone or gender modifiers** - multi-codepoint sequences that may not render as a single character
- **Variation selector emojis** (e.g., `❄️`, `☀️`) - width inconsistencies between terminals
- **ZWJ sequences** - composite emojis that may break in older terminals
- **Emoji 15.0+** - limited font/terminal support as of 2024

All included emojis have been verified to render correctly as full-width characters in modern terminal emulators (kitty, alacritty, iTerm2, GNOME Terminal, Windows Terminal).

## Options

### `--no-emoji`

For terminals without emoji support, use `--no-emoji` to generate `adjective-noun` pairs instead:

```
$ tmux-session-namegen --no-emoji
vibrant-fjord

$ tmux-session-namegen --no-emoji
eloquent-beacon

$ tmux-session-namegen --no-emoji
agile-sentinel
```

This mode uses **196 nouns** (animals, nature, food, celestial objects, buildings, and more), giving **29,204 unique combinations**.

## Building

Requires [Rust](https://www.rust-lang.org/tools/install) (edition 2021+).

```bash
cargo build --release
```

The binary is at `target/release/tmux-session-namegen`.

## Installation

Copy the binary somewhere on your `$PATH`:

```bash
cp target/release/tmux-session-namegen ~/.local/bin/
```

## Usage with tmux

Create a new session with a random name:

```bash
tmux new-session -s "$(tmux-session-namegen)"
```

Or without emojis:

```bash
tmux new-session -s "$(tmux-session-namegen --no-emoji)"
```

Add to your shell config (e.g., `.bashrc` or `.zshrc`) for automatic naming:

```bash
alias tn='tmux new-session -s "$(tmux-session-namegen)"'
alias tnn='tmux new-session -s "$(tmux-session-namegen --no-emoji)"'
```

## License

Apache License 2.0. See [LICENSE](LICENSE).
