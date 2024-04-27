use ratatui::{backend::CrosstermBackend, Terminal};

mod tui;

pub enum Event {
    Quit,
    Error,
    Tick,
    Render,
    Key(KeyEvent),
}

fn update(app: &mut App, event: Event) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            Char('j') => app.counter += 1,
            Char('k') => app.counter -= 1,
            Char('q') => app.should_quit = true,
            _ => {}
        }
    }
    Ok(())
}

async fn run() => Result<()> {
    let mut events = tui::EventHandler::new();

    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app = App { counter: 0, should_quit: false};

    loop {
        let event = events.next().await?;

        update(&mut app, event)?;

        t.draw(|f| {
            ui(f, &app);
        })?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    startup()?;

    let result = run();

    shutdown()?;

    result?;

    Ok(())
}
