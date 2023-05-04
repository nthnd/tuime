use anyhow::Result;
use clap::Parser;
use config::Config;

use std::time::Duration;
use tokio::time::interval;

use crossterm::{
    cursor,
    event::{Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use futures::{FutureExt, StreamExt};

mod args;
mod config;
mod display;
mod error;

use args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    enable_raw_mode()?;
    crossterm::execute! {
        std::io::stdout(),
        EnterAlternateScreen,
        cursor::Hide
    }?;

    let cfg = Config::new(&args.font, args.colors.clone());
    let mut reader = EventStream::new();
    let mut interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            maybe_event = reader.next().fuse() => {
                if matches!(maybe_event, Some(Ok(Event::Key(key))) if key == KeyCode::Char('q').into()) {break}
            }

            _ = interval.tick() => {
                if let Err(e) = display::print_time(&args, &cfg) {
                   println!("{:?}", e)
                }
            }

        }
    }

    disable_raw_mode()?;
    crossterm::execute! {
        std::io::stdout(),
        LeaveAlternateScreen,
        cursor::Show
    }?;
    Ok(())
}
