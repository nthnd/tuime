use anyhow::Result;
use clap::Parser;

use std::time::Duration;
use tokio::time::interval;

use crossterm::{
    cursor,
    event::{Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use futures::{FutureExt, StreamExt};

use crate::args::Args;
use crate::config::Config;

mod args;
mod config;
mod display;
mod error;

mod colors;
mod fonts;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    enable_raw_mode()?;
    crossterm::execute! {
        std::io::stdout(),
        EnterAlternateScreen,
        cursor::Hide
    }?;

    let cfg = Config::new(&args.font, &args.colors);
    let mut reader = EventStream::new();
    let mut interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            maybe_event = reader.next().fuse() => {
                if let Some(Ok(Event::Key(q))) = maybe_event {
                    if args.screensaver | (q.code == KeyCode::Char('q')) {
                        break;
                    }
                }
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
