use crate::app::{App, AppResult, Direction};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Enter => {
            app.add_new_line();
        }
        KeyCode::Backspace => {
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
                app.insert_char(c)
            }
        }
    }
    Ok(())
}
