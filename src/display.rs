use std::io::Write;

use cfonts::{render, Align, Colors, Fonts, Options};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};

use crate::{args::Args, error::TuimeError};

fn color_from_str(s: String) -> Colors {
    match s.as_str() {
        "Black" => Colors::Black,
        "Red" => Colors::Red,
        "Green" => Colors::Green,
        "Yellow" => Colors::Yellow,
        "Blue" => Colors::Blue,
        "Magenta" => Colors::Magenta,
        "Cyan" => Colors::Cyan,
        "White" => Colors::White,
        "Gray" => Colors::Gray,
        "RedBright" => Colors::RedBright,
        "GreenBright" => Colors::GreenBright,
        "YellowBright" => Colors::YellowBright,
        "BlueBright" => Colors::BlueBright,
        "MagentaBright" => Colors::MagentaBright,
        "CyanBright" => Colors::CyanBright,
        "WhiteBright" => Colors::WhiteBright,
        "Candy" => Colors::Candy,
        _ => Colors::System,
    }
}

fn font_from_str(s: &str) -> Fonts {
    match s {
        "Console" => Fonts::FontConsole,
        "Block" => Fonts::FontBlock,
        "SimpleBlock" => Fonts::FontSimpleBlock,
        "Simple" => Fonts::FontSimple,
        "3d" => Fonts::Font3d,
        "Simple3d" => Fonts::FontSimple3d,
        "Chrome" => Fonts::FontChrome,
        "Huge" => Fonts::FontHuge,
        "Shade" => Fonts::FontShade,
        "Slick" => Fonts::FontSlick,
        "Grid" => Fonts::FontGrid,
        "Pallet" => Fonts::FontPallet,
        "Tiny" => Fonts::FontTiny,
        _ => Fonts::FontBlock,
    }
}

pub fn get_time_str(args: &Args) -> String {
    let time = chrono::Local::now().format(&args.format).to_string();

    let time_str = render(Options {
        text: time,
        align: Align::Center,
        font: font_from_str(&args.font),
        gradient: args.gradient.clone(),
        colors: args
            .colors
            .clone()
            .into_iter()
            .map(color_from_str)
            .collect::<Vec<Colors>>(),
        ..Options::default()
    });

    time_str.text
}

pub fn print_time(args: &Args) -> Result<(), TuimeError> {
    let time_str: Vec<String> = get_time_str(args)
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
