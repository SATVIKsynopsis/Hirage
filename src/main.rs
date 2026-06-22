mod app;
mod compiler;
mod event;
mod function_view;
mod parser;
mod project;
mod state;
mod ui;

use std::io;

use color_eyre::Result;

use crossterm::{
    event::{Event, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    if !app.files.is_empty() {
        app.open_selected_file();
    }

    loop {
        terminal.draw(|f| {
            ui::render(f, &app);
        })?;

        if app.should_quit {
            break;
        }

        if let Event::Key(key) = read()? {
            event::handle_key(&mut app, key);
        }
    }

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
