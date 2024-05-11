use ratatui::{backend::CrosstermBackend, Terminal};

use rust_edit::app::{App, AppResult};
use rust_edit::event::{Event, EventHandler};
use rust_edit::handler::handle_key_events;
use rust_edit::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    tui.draw(&mut app)?;

    while app.running {
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app)?;
                tui.draw(&mut app)?;
            }
            Event::Mouse(_) => tui.draw(&mut app)?,
            Event::Resize(_, _) => tui.draw(&mut app)?,
        }
    }

    tui.exit()?;
    Ok(())
}
