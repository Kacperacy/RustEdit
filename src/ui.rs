use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const RELATIVE_LINES: bool = true;

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let content_width = (frame.size().width - 5) as usize;
    let numbers_width = std::cmp::max((app.content.len() as f64).log10().ceil() as usize, 4);

    let pos = app.get_cursor_positon();

    let content_lines: Vec<Line> = app
        .content
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if i == pos.y {
                Line::from(format!("{:<content_width$}", s))
                    .style(Style::default().bg(Color::Rgb(64, 64, 96)))
            } else {
                Line::from(format!("{}", s))
            }
        })
        .collect();

    let numbers = if RELATIVE_LINES {
        if app.content.len() == 1 {
            vec![1]
        } else {
            (1..=pos.y)
                .rev()
                .chain(std::iter::once(pos.y))
                .chain(1..=app.content.len().saturating_sub(pos.y + 1))
                .collect::<Vec<_>>()
        }
    } else {
        (1..=app.content.len()).collect::<Vec<_>>()
    };

    let line_numbers: Vec<Line> = numbers
        .iter()
        .enumerate()
        .map(|(e, &i)| {
            if e == pos.y {
                Line::from(format!("{:<numbers_width$} ", i))
                    .style(Style::default().fg(Color::Rgb(96, 128, 196)))
            } else {
                Line::from(format!("{:>numbers_width$} ", i))
            }
        })
        .collect();

    let filename_status = Line::from(format!("Filename: {}", app.opened_filename))
        .left_aligned()
        .style(Style::default().bg(Color::Rgb(128, 192, 255)).bold());

    let cursor_position_status = Line::from(format!(" {:>2}:{:<2} ", pos.y + 1, pos.x + 1,))
        .right_aligned()
        .style(Style::default().bg(Color::Rgb(128, 192, 255)).bold());

    let status_line: Line = if app.is_prompt {
        Line::from(format!("{:<}", app.prompt))
    } else {
        Line::from("Press Ctrl + C to quit, Ctrl + S to save.").centered()
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.size());

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(layout[1]);

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
        Paragraph::new(line_numbers)
            .style(
                Style::default()
                    .fg(Color::Rgb(64, 96, 128))
                    .bg(Color::Rgb(32, 32, 64)),
            )
            .scroll((app.cursor_offset.y as u16, app.cursor_offset.x as u16)),
        content_layout[0],
    );

    frame.render_widget(
        Paragraph::new(content_lines)
            .style(
                Style::default()
                    .fg(Color::Rgb(128, 192, 255))
                    .bg(Color::Rgb(32, 32, 64)),
            )
            .scroll((app.cursor_offset.y as u16, app.cursor_offset.x as u16)),
        content_layout[1],
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

    frame.render_widget(status_line, layout[3]);

    frame.set_cursor(
        (app.cursor_position.x + (if app.is_prompt { 0 } else { numbers_width + 1 })) as u16,
        app.cursor_position.y as u16 + 1,
    );
}
