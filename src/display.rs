use std::io::Write;

use cfonts::{render, Align, Options};
use chrono::{FixedOffset, Utc};
use crossterm::terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};

use crate::colors::ColorsVecNTWrapper;
use crate::{args::Args, config::Config, error::TuimeError};

pub fn get_time_str(args: &Args, cfg: &Config) -> String {
    let time = match args.utc_offset.as_ref() {
        None => chrono::Local::now().format(&args.format).to_string(),
        Some(offset) => Utc::now()
            .with_timezone(&FixedOffset::east_opt(*offset).unwrap())
            .format(&args.format)
            .to_string(),
    };

    let time_str = render(Options {
        text: time,
        align: Align::Center,
        font: cfg.font.to_owned().into(),
        gradient: args.gradient.clone(),
        colors: ColorsVecNTWrapper(cfg.color.to_owned()).into(),
    //    debug: true,
    //    debug_level: 10,
        ..Options::default()
    });
 
    time_str.text
}

pub fn print_time(args: &Args, cfg: &Config) -> Result<(), TuimeError> {
    let time_str: Vec<String> = get_time_str(args, cfg)
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect();

    let highest_col = time_str.len();
    let (width, height) = terminal::size().map_err(|_| TuimeError::WindowTooSmall(0, 0))?;
    let (width, height) = (width as usize, height as usize);

    if width < 20 || height < 5 {
        return Err(TuimeError::WindowTooSmall(width, height));
    };
    let mut stdout = std::io::stdout();
    
    stdout.queue(BeginSynchronizedUpdate).unwrap();
    if height != cfg.height.get() || width != cfg.width.get() {
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        cfg.height.set(height);
        cfg.width.set(width);
    }
    for (line_nr, line) in time_str.iter().enumerate() {
        stdout
        .queue(cursor::MoveTo(
            0,
            (line_nr + (height - highest_col) / 2)
            .try_into()
            .map_err(|_| TuimeError::WindowTooSmall(width, height))?,
        ))
        .unwrap();
        crossterm::queue!(stdout,  Print(line), terminal::Clear(terminal::ClearType::UntilNewLine)).unwrap();
        //stdout.flush().unwrap();
    }
    stdout.queue(EndSynchronizedUpdate).unwrap()
        .flush().unwrap();
    Ok(())
}
