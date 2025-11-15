use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub fn handle_events(timeout: Duration) -> std::io::Result<Option<KeyEvent>> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            return Ok(Some(key));
        }
    }
    Ok(None)
}