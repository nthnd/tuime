use cfonts::{Fonts, Colors};

pub struct Config {
    pub font: Fonts,
    pub color: Vec<Colors>,
}

impl Config {
    pub fn new(font: &str, color: Vec<String>) -> Self {
        Self {
            font: font_from_str(font),
            color: color.into_iter().map(color_from_str).collect(),
        }
    }
}

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

