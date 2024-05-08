use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let mut content_lines: Vec<Line> = app.content.iter().map(|s| s.as_str().into()).collect();

    content_lines.push(Line::from(format!("{}", app.cursor_position.x)));
    content_lines.push(Line::from(format!("{}", app.cursor_position.y)));

    frame.render_widget(
        Paragraph::new(content_lines)
            .block(
                Block::bordered()
                    .title("RustEdit")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::LightBlue).bg(Color::Black)),
        frame.size(),
    );
    frame.set_cursor(
        app.cursor_position.x as u16 + 1,
        app.cursor_position.y as u16 + 1,
    );
}
