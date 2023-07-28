use pomo::app::{App, AppResult};
use pomo::event::{Event, EventHandler};
use pomo::handler::handle_key_events;
use pomo::tui::Tui;
use std::fs;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // look for config_file
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_file_path: PathBuf = [
        home_dir,
        ".config".into(),
        "pomodorors".into(),
        "config".into(),
    ]
    .iter()
    .collect();
    let file = fs::read_to_string(&config_file_path);
    let mut app: App;

    match file {
        Ok(val) => app = App::new_from_file(val),
        Err(err) => {
            if err.kind() == ErrorKind::PermissionDenied {
                panic!("Application doesn't have permision to read config.");
            }
            if err.kind() == ErrorKind::NotFound {
                let mut temp = App::new();
                temp.default_config();
            }
            app = App::new();
        }
    }

    // Create an application.

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
