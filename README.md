# Cli pomodoro timer

I created this as one of my first rust projects along side with [text-to-ascii-art](https://crates.io/crates/text-to-ascii-art), which is a library for
making larger text in terminal using ascii-art. I hope you find this useful ❤️ .

# Install

```bash
cargo install --git https://github.com/osmak1234/pomodoro
```

# Controls

t - show tooltip <br>

d - sound effects <br>
r - reset current segment <br>
s - skip current segment<br>
m - display only minutes

⬆️ - increase work duration<br>
⬇️ - decrease work duration<br>
⬅️ - increase pause duration<br>
➡️ - decrease pause duration<br>

# Defaults

25 min - work <br>
5 min - pause <br>
<br>
tooltip - hidden <br>
display only minutes - false

# Usage

I use this in a small floating window, here is the launch for the kitty terminal.

```bash
kitty --class "floating" -o allow_remote_control=yes  -o remember_window_size=nom  -o initial_window_width=500 -o initial_window_height=350 pomo
```

# Roadmap

- [x] Sounds
  - [x] Toggle sounds
- [x] Adjustable time
  - [ ] Set defaults permanently
- [x] Big text
- [x] Work/Pause indicator
- [x] Paused timer indicator
- [x] Display only minutes toggle
