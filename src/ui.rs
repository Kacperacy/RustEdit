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

    let filename_status = Line::from(format!("Filename: {}", app.opened_filename))
        .left_aligned()
        .style(Style::default().bg(Color::Rgb(128, 192, 255)).bold());

    let cursor_position_status = Line::from(format!(
        " {:>2}:{:<2} ",
        app.cursor_position.y + 1,
        app.cursor_position.x + 1,
    ))
    .right_aligned()
    .style(Style::default().bg(Color::Rgb(128, 192, 255)).bold());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.size());

    let status_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[2]);

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
        Paragraph::new(content_lines)
            .style(
                Style::default()
                    .fg(Color::Rgb(128, 192, 255))
                    .bg(Color::Rgb(32, 32, 64)),
            )
            .scroll((app.cursor_offset.y as u16, app.cursor_offset.x as u16)),
        layout[1],
    );

    frame.render_widget(
        Paragraph::new(filename_status).style(
            Style::default()
                .fg(Color::Rgb(16, 16, 16))
                .bg(Color::Rgb(92, 92, 128)),
        ),
        status_bar_layout[0],
    );

    frame.render_widget(
        Paragraph::new(cursor_position_status).style(
            Style::default()
                .fg(Color::Rgb(16, 16, 16))
                .bg(Color::Rgb(92, 92, 128)),
        ),
        status_bar_layout[1],
    );

    frame.render_widget(Line::from("Press Ctrl + C to quit").centered(), layout[3]);

    frame.set_cursor(
        app.cursor_position.x as u16,
        app.cursor_position.y as u16 + 1,
    );
}
