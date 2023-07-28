# Cli pomodoro timer

I created this as one of my first rust projects along side with [text-to-ascii-art](https://crates.io/crates/text-to-ascii-art), which is a library for
making larger text in terminal using ascii-art. I hope you find this useful ❤️ .


https://github.com/osmak1234/pomodoro/assets/91377215/19eb67fe-6e43-40d6-9c8d-1798a3891f7d

# Install

```bash
cargo install --git https://github.com/osmak1234/pomodoro
```




# Controls

t - show tooltip <br>

s - sound effects <br>
r - restart <br>
f - skip <br>
m - display only minutes<br>
w - save config to file<br>
d - default in app, file<br>

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

- [x] Finished project

- [x] README video showcase

- [x] Save config permanently
  
- [x] Sounds
  - [x] Toggle sound

- [x] Adjustable time
  - [x] Set defaults permanently
  - [x] Store values in .config/pomodorors

- [x] Big text
- [x] Work/Pause indicator
- [x] Paused timer indicator
- [x] Display only minutes toggle
