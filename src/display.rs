// TODO: handle all unwraps

use std::io::Write;

use cfonts::{render, Align, Options};
use crossterm::{cursor, style::Print, terminal};

pub fn get_time_str() -> String {
    let time = chrono::Local::now().format("%H:%M").to_string();

    let time_str = render(Options {
        text: time,
        align: Align::Center,
        ..Options::default()
    });

    time_str.text
}

pub fn print_time() {
    let time_str: Vec<String> = get_time_str()
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect();

    let highest_col = time_str.iter().count();
    let height = terminal::size().unwrap().1 as usize;

    let mut stdout = std::io::stdout();

    crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::All),).unwrap();
    for (line_nr, line) in time_str.iter().enumerate() {
        crossterm::queue!(
            stdout,
            cursor::MoveTo(0, (line_nr + (height - highest_col) / 2) as u16),
            Print(line),
        )
        .unwrap();
    }

    stdout.flush().unwrap();
}
