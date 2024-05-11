use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let content_lines: Vec<Line> = app.content.iter().map(|s| s.as_str().into()).collect();

    let status_bar = Line::from(format!(
        " {:>2}:{:<2} ",
        app.cursor_position.y + 1,
        app.cursor_position.x + 1
    ))
    .right_aligned()
    .style(Style::default().bg(Color::Rgb(128, 192, 255)).bold());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.size());

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("RustEdit")
            .title_alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::Rgb(128, 192, 255))
                    .bg(Color::Rgb(32, 32, 64))
                    .bold(),
            ),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(content_lines).style(
            Style::default()
                .fg(Color::Rgb(128, 192, 255))
                .bg(Color::Rgb(32, 32, 64)),
        ),
        layout[1],
    );

    frame.render_widget(
        Paragraph::new(status_bar).style(
            Style::default()
                .fg(Color::Rgb(16, 16, 16))
                .bg(Color::Rgb(92, 92, 128)),
        ),
        layout[2],
    );

    frame.set_cursor(
        app.cursor_position.x as u16,
        app.cursor_position.y as u16 + 1,
    );
}
