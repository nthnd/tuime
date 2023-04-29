use std::io::Write;

use cfonts::{render, Align, Colors, Fonts, Options};
use chrono::{FixedOffset, Utc};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};

use crate::{args::Args, error::TuimeError};

fn color_from_str(s: String) -> Colors {
    match s.to_lowercase().as_str() {
        "black" => Colors::Black,
        "red" => Colors::Red,
        "green" => Colors::Green,
        "yellow" => Colors::Yellow,
        "blue" => Colors::Blue,
        "magenta" => Colors::Magenta,
        "cyan" => Colors::Cyan,
        "white" => Colors::White,
        "gray" => Colors::Gray,
        "redbright" => Colors::RedBright,
        "greenbright" => Colors::GreenBright,
        "yellowbright" => Colors::YellowBright,
        "bluebright" => Colors::BlueBright,
        "magentabright" => Colors::MagentaBright,
        "cyanbright" => Colors::CyanBright,
        "whitebright" => Colors::WhiteBright,
        "candy" => Colors::Candy,
        _ => Colors::System,
    }
}

fn font_from_str(s: &str) -> Fonts {
    match s.to_lowercase().as_str() {
        "console" => Fonts::FontConsole,
        "block" => Fonts::FontBlock,
        "simpleblock" => Fonts::FontSimpleBlock,
        "simple" => Fonts::FontSimple,
        "3d" => Fonts::Font3d,
        "simple3d" => Fonts::FontSimple3d,
        "chrome" => Fonts::FontChrome,
        "huge" => Fonts::FontHuge,
        "shade" => Fonts::FontShade,
        "slick" => Fonts::FontSlick,
        "grid" => Fonts::FontGrid,
        "pallet" => Fonts::FontPallet,
        "tiny" => Fonts::FontTiny,
        _ => Fonts::FontBlock,
    }
}

pub fn get_time_str(args: &Args) -> String {
    let time = match args.utc_offset.as_ref() {
        None => chrono::Local::now().format(&args.format).to_string(),
        Some(offset) => Utc::now()
            .with_timezone(&FixedOffset::east_opt(*offset).unwrap())
            .format(&args.format)
            .to_string(),
    };

    // let time = chrono::Local::now().format(&args.format).to_string();

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
