use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char(' ') => app.toggle_stopwatch(), // timer handlers

        KeyCode::Up => app.change_duration(true, true),
        KeyCode::Down => app.change_duration(false, true),
        KeyCode::Left => app.change_duration(true, false),
        KeyCode::Right => app.change_duration(false, false),
        KeyCode::Char('t') => app.toggle_tooltip(),
        KeyCode::Char('r') => app.stopwatch.restart(),
        KeyCode::Char('s') => app.skip(),
        KeyCode::Char('m') => app.toggle_only_minutes(),

        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
