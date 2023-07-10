# Cli pomodoro timer

I created this as one of my first rust projects along side with [text-to-ascii-art](https://crates.io/crates/text-to-ascii-art), which is a library for
making large text in terminal using ascii-art

# Install

```bash
cargo install --git https://github.com/osmak1234/pomodoro
```

# Controls

t - show tooltip

r - reset current segment
s - skip current segment

⬆️ - increase work duration
⬇️ - decrease work duration
⬅️ - increase pause duration
➡️-- decrease pause duration

# Defaults

25 min - work
5 min - pause

# Usage

I use this in a small floating window, usually over my spotify, here is the launch for kitty

```bash
kitty --class "floating" -o allow_remote_control=yes  -o remember_window_size=nom  -o initial_window_width=500 -o initial_window_height=350 pomodoro
```

# Roadmap

- [x] Sounds
- [x] Adjustable time
  - [ ] Set defaults permanently
- [x] Big text
- [x] Work/Pause indicator
- [x] Paused timer indicator
