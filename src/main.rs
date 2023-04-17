use anyhow::Result;

use std::time::Duration;
use tokio::time::interval;

use crossterm::{
    cursor,
    event::{Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use futures::{FutureExt, StreamExt};

mod display;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;

    let mut reader = EventStream::new();
    let mut interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            maybe_event = reader.next().fuse() => {
                if let Some(Ok(event)) = maybe_event {
                    if Event::Key(KeyCode::Char('q').into()) == event {
                        break
                    }
                }
            }

            _ = interval.tick() => {
                display::print_time()
            }

        }
    }

    disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}
