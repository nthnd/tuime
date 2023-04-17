use std::io::Write;

use cfonts::{render, Align, Options};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};

use crate::{args::Args, error::TuimeError};

pub fn get_time_str(args: &Args) -> String {
    let time = chrono::Local::now().format(&args.format).to_string();

    let time_str = render(Options {
        text: time,
        align: Align::Center,
        ..Options::default()
    });

    time_str.text
}

pub fn print_time(args: &Args) -> Result<(), TuimeError> {
    let time_str: Vec<String> = get_time_str(&args)
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

    crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::All),).unwrap();
    for (line_nr, line) in time_str.iter().enumerate() {
        stdout
            .queue(cursor::MoveTo(
                0,
                (line_nr + (height - highest_col) / 2)
                    .try_into()
                    .map_err(|_| TuimeError::WindowTooSmall(width, height))?,
            ))
            .unwrap();
        crossterm::queue!(stdout, Print(line),).unwrap();
    }

    stdout.flush().unwrap();
    Ok(())
}
