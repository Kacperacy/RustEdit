use crate::app::{App, AppResult, Direction};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.modifiers {
        KeyModifiers::CONTROL | KeyModifiers::ALT => {
            if key_event.code == KeyCode::Char('c') || key_event.code == KeyCode::Char('C') {
                app.quit();
            }
            if key_event.code == KeyCode::Char('s') || key_event.code == KeyCode::Char('S') {
                app.save_to_file();
            }
            if key_event.code == KeyCode::Left {
                app.jump_at_start_line();
            }
            if key_event.code == KeyCode::Right {
                app.jump_at_end_line();
            }
        }
        _ => match key_event.code {
            KeyCode::Esc => {
                app.exit_prompt();
            }
            KeyCode::Enter => {
                app.set_dirty();
                app.add_new_line();
            }
            KeyCode::Backspace => {
                app.set_dirty();
                app.pop_char();
            }
            KeyCode::Left => {
                app.move_cursor(Direction { x: -1, y: 0 });
            }
            KeyCode::Right => {
                app.move_cursor(Direction { x: 1, y: 0 });
            }
            KeyCode::Up => {
                app.move_cursor(Direction { x: 0, y: 1 });
            }
            KeyCode::Down => {
                app.move_cursor(Direction { x: 0, y: -1 });
            }
            _ => {
                if let KeyCode::Char(c) = key_event.code {
                    app.set_dirty();
                    app.insert_char(c)
                }
            }
        },
    };

    Ok(())
}
