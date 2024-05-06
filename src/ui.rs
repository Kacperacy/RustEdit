use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("{}", app.content))
            .block(
                Block::bordered()
                    .title("RustEdit")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::LightBlue).bg(Color::Black)),
        frame.size(),
    );
    frame.set_cursor(app.cursor_position.x + 1, app.cursor_position.y + 1);
}
