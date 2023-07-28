use std::{error, fs, io::Write, path::PathBuf};
extern crate stopwatch;
use notify_rust::Notification;
use rodio::{source::Source, Decoder, OutputStream};
use serde::{Deserialize, Serialize};
use serde_json;
use stopwatch::Stopwatch;
use text_to_ascii_art::convert;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Config file json
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub work_duration: i32,
    pub pause_duration: i32,
    pub only_minutes: bool,
    pub sound: bool,
}

/// Application.
pub struct App {
    pub running: bool,
    pub pause: bool,
    pub timer: String,
    pub work_duration: i32,
    pub pause_duration: i32,
    pub stopwatch: Stopwatch,
    pub tooltip: String,
    pub button: String,
    pub only_minutes: bool,
    pub sound: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            work_duration: 25 * 60,
            pause_duration: 5 * 60,
            pause: false,
            timer: "".to_string(),
            stopwatch: Stopwatch::start_new(),
            running: true,
            tooltip: "".to_string(),
            button: "".to_string(),
            only_minutes: false,
            sound: "󰕾".to_string(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_config(&mut self) {
        let default_config = r#"
{
     "work_duration": 1500,
     "pause_duration": 300,
     "only_minutes": false,
     "sound": false
}
"#;
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let config_file_path: PathBuf = [
            home_dir.clone(),
            ".config".into(),
            "pomodorors".into(),
            "config".into(),
        ]
        .iter()
        .collect();

        let _ = fs::create_dir(
            [home_dir, ".config".into(), "pomodorors".into()]
                .iter()
                .collect::<PathBuf>(),
        );
        let mut config_file = fs::File::create(config_file_path).expect("Failed to create file");

        config_file.write_all(default_config.as_bytes()).unwrap();
        self.work_duration = 1500;
        self.pause_duration = 300;
        self.sound = "󰖁".to_string();
        self.only_minutes = false;
    }

    pub fn write_config(&self) {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let config_file_path: PathBuf = [
            home_dir,
            ".config".into(),
            "pomodorors".into(),
            "config".into(),
        ]
        .iter()
        .collect();

        let mut config_file = fs::File::create(config_file_path).expect("Failed to create file");

        let config_to_save = format!(
            r#"
{{
    "work_duration": {},
    "pause_duration": {},
    "only_minutes": {},
    "sound": {} 
}}
"#,
            self.work_duration,
            self.pause_duration,
            self.only_minutes,
            self.sound == *"󰕾",
        );

        config_file
            .write_all(config_to_save.as_bytes())
            .expect("Writing config failed");
    }

    pub fn new_from_file(config: String) -> Self {
        let config: AppConfig = serde_json::from_str(&config).unwrap();

        Self {
            running: true,
            pause: false,
            timer: "".to_string(),
            work_duration: config.work_duration,
            pause_duration: config.pause_duration,
            stopwatch: Stopwatch::start_new(),
            button: "".to_string(),
            tooltip: "".to_string(),
            only_minutes: config.only_minutes,
            sound: if config.sound {
                "󰕾".to_string()
            } else {
                "󰖁".to_string()
            },
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.pause_check();
        self.update_clock();
        if self.work_duration == 0 && self.pause_duration == 0 {
            self.quit();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_only_minutes(&mut self) {
        self.only_minutes = match self.only_minutes {
            true => false,
            false => true,
        }
    }

    pub fn pause_check(&mut self) {
        let second_countdown = if self.pause {
            self.pause_duration
        } else {
            self.work_duration
        } - self.stopwatch.elapsed().as_secs() as i32;
        if second_countdown < 1 {
            self.stopwatch.restart();
            let message = match self.pause {
                true => format!(
                    "End of pause
work: {} min ",
                    self.work_duration / 60
                ),
                false => format!(
                    "End of work
pause: {} min ",
                    self.pause_duration / 60
                ),
            };
            let _ = Notification::new().summary(&message).show();

            if self.sound == *"󰕾" {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let my_slice = std::io::Cursor::new(include_bytes!("../notify.mp3").as_ref());
                let source = Decoder::new(my_slice).unwrap();
                let _sound_result = stream_handle.play_raw(source.convert_samples());
                std::thread::sleep(std::time::Duration::from_millis(900));
            }
            self.pause ^= true;
        }
    }

    pub fn skip(&mut self) {
        if self.pause {
            let previous_duration = self.pause_duration;
            self.pause_duration = 0;
            self.pause_check();
            self.pause_duration = previous_duration;
        } else {
            let previous_duration = self.work_duration;
            self.work_duration = 0;
            self.pause_check();
            self.work_duration = previous_duration;
        }
    }

    pub fn update_clock(&mut self) {
        let second_countdown = if self.pause {
            self.pause_duration
        } else {
            self.work_duration
        } - self.stopwatch.elapsed().as_secs() as i32;
        let (seconds, minutes) = (second_countdown % 60, second_countdown / 60);
        let time: String = if self.only_minutes {
            format!("{:2}", minutes)
        } else {
            format!("{:2}:{:2}", minutes, seconds)
        };
        match convert(time) {
            Ok(text) => self.timer = text,
            Err(err) => panic!("{:?}", err),
        }
    }

    pub fn toggle_stopwatch(&mut self) {
        match self.stopwatch.is_running() {
            true => {
                self.stopwatch.stop();
                self.button = " 
 ██╗ ██╗
 ██║ ██║
 ██║ ██║
 ╚═╝ ╚═╝
"
                .to_string()
            }
            false => {
                self.stopwatch.start();
                self.button = "".to_string()
            }
        }
        if self.sound == *"󰕾" {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let my_slice = std::io::Cursor::new(include_bytes!("../click.mp3").as_ref());
            let source = Decoder::new(my_slice).unwrap();
            let _sound_result = stream_handle.play_raw(source.convert_samples());
            std::thread::sleep(std::time::Duration::from_millis(220));
        }
    }

    pub fn toggle_sound(&mut self) {
        match self.sound.as_str() {
            "󰖁" => self.sound = "󰕾".to_string(),
            "󰕾" => self.sound = "󰖁".to_string(),
            _ => {}
        }
    }

    pub fn change_duration(&mut self, increase: bool, work: bool) {
        match (work, increase) {
            (true, true) => self.work_duration += 60,
            (true, false) => {
                if self.work_duration != 0 {
                    self.work_duration -= 60
                }
            }
            (false, true) => self.pause_duration += 60,
            (false, false) => {
                if self.pause_duration != 0 {
                    self.pause_duration -= 60
                }
            }
        }
        self.toggle_tooltip();
        self.toggle_tooltip();
    }

    pub fn toggle_tooltip(&mut self) {
        if self.tooltip.is_empty() {
            self.tooltip = format!(
                "












  {} min
󱦲            Work           󱦳
 Increas                    Decrease 
󱦱            Pause          󱦰
  {} min
    
     {}

    s - sound effects
    r - restart 
    f - skip
    m - display only minutes
    w - save config to file
    d - default in app, file
",
                self.work_duration / 60,
                self.pause_duration / 60,
                self.sound
            );
        } else {
            self.tooltip = "".to_string();
        }
    }
}
