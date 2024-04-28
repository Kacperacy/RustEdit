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

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
