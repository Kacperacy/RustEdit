use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Enter => {
            app.append_char('\n');
        }
        KeyCode::Backspace => {
            app.pop_char();
        }
        _ => {
            if let KeyCode::Char(c) = key_event.code {
                app.append_char(c)
            }
        }
    }
    Ok(())
}
