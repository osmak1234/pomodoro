use tui::{
    backend::Backend,
    layout::Alignment,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "{}
{}
        {}",
            app.timer, app.button, app.tooltip
        ))
        .block(
            Block::default()
                .title(if app.pause { " pause " } else { " work " })
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default())
        .alignment(Alignment::Center),
        frame.size(),
    );
}
