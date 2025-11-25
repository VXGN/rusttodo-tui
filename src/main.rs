mod app;
mod event;
mod todo;
mod ui;

use app::{AddingField, App, InputMode};
use crossterm::{
    event::{KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::handle_events;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Some(key) = handle_events(Duration::from_millis(100))? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            handle_input(app, key);
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_input(app: &mut App, key: crossterm::event::KeyEvent) {
    match &app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Adding(_) => handle_adding_mode(app, key),
        InputMode::ViewingDetails if key.code == KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_normal_mode(app: &mut App, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('a') => app.input_mode = InputMode::Adding(AddingField::Title),
        KeyCode::Char('d') => app.delete_selected(),
        KeyCode::Char(' ') => app.toggle_selected(),
        KeyCode::Down | KeyCode::Char('j') => app.next_item(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_item(),
        KeyCode::Tab => app.next_tab(),
        KeyCode::Enter => app.input_mode = InputMode::ViewingDetails,
        _ => {}
    }
}

fn handle_adding_mode(app: &mut App, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.input_buffer.clear();
            app.description_buffer.clear();
            app.priority_index = 1;
        }
        KeyCode::Enter => {
            app.add_todo();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Tab => {
            app.input_mode = match &app.input_mode {
                InputMode::Adding(AddingField::Title) => {
                    InputMode::Adding(AddingField::Description)
                }
                InputMode::Adding(AddingField::Description) => {
                    InputMode::Adding(AddingField::Title)
                }
                _ => InputMode::Adding(AddingField::Title),
            };
        }
        KeyCode::Char(c) => match &app.input_mode {
            InputMode::Adding(AddingField::Title) => app.input_buffer.push(c),
            InputMode::Adding(AddingField::Description) => app.description_buffer.push(c),
            _ => {}
        },
        KeyCode::Backspace => match &app.input_mode {
            InputMode::Adding(AddingField::Title) => {
                app.input_buffer.pop();
            }
            InputMode::Adding(AddingField::Description) => {
                app.description_buffer.pop();
            }
            _ => {}
        },
        KeyCode::Up if app.priority_index > 0 => app.priority_index -= 1,
        KeyCode::Down if app.priority_index < 2 => app.priority_index += 1,
        _ => {}
    }
}
