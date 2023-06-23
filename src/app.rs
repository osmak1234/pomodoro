use std::error;

use std::time::{Duration, Instant};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// timer
    pub timer: i32,
    /// start time
    pub start_time: Instant,
    /// duration
    pub duration: i32,

    pub stoped: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            duration: 25 * 60,
            stoped: true,
            running: true,
            timer: 25 * 60,
            start_time: Instant::now(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.set_timer();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle(&mut self) {
        if self.stoped {
            self.stoped = false;
        } else {
            self.stoped = true;
        }
    }

    pub fn set_timer(&mut self) {
        self.timer = self.duration - self.start_time.elapsed().as_secs() as i32;
    }

    pub fn increment_timer(&mut self) {
        if let Some(res) = self.timer.checked_add(1) {
            self.timer = res;
        }
    }
}
