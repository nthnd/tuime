use std::cell::Cell;
use crate::colors::Colors;
use crate::fonts::Fonts;

pub struct Config {
    pub font: Fonts,
    pub color: Vec<Colors>,
    pub height: Cell<usize>,
    pub width: Cell<usize>,
}

impl Config {
    pub fn new(font: &Fonts, color: &[Colors]) -> Self {
        Self {
            font: font.to_owned(),
            color: color.to_owned(),
            width: Cell::new(0),
            height: Cell::new(0)
        }
    }
}

// fn color_from_str(s: String) -> Colors {
//     match s.to_lowercase().as_str() {
//         "black" => Colors::Black,
//         "red" => Colors::Red,
//         "green" => Colors::Green,
//         "yellow" => Colors::Yellow,
//         "blue" => Colors::Blue,
//         "magenta" => Colors::Magenta,
//         "cyan" => Colors::Cyan,
//         "white" => Colors::White,
//         "gray" => Colors::Gray,
//         "redbright" => Colors::RedBright,
//         "greenbright" => Colors::GreenBright,
//         "yellowbright" => Colors::YellowBright,
//         "bluebright" => Colors::BlueBright,
//         "magentabright" => Colors::MagentaBright,
//         "cyanbright" => Colors::CyanBright,
//         "whitebright" => Colors::WhiteBright,
//         "candy" => Colors::Candy,
//         _ => Colors::System,
//     }
// }
//
// fn font_from_str(s: &str) -> Fonts {
//     match s.to_lowercase().as_str() {
//         "console" => Fonts::FontConsole,
//         "block" => Fonts::FontBlock,
//         "simpleblock" => Fonts::FontSimpleBlock,
//         "simple" => Fonts::FontSimple,
//         "3d" => Fonts::Font3d,
//         "simple3d" => Fonts::FontSimple3d,
//         "chrome" => Fonts::FontChrome,
//         "huge" => Fonts::FontHuge,
//         "shade" => Fonts::FontShade,
//         "slick" => Fonts::FontSlick,
//         "grid" => Fonts::FontGrid,
//         "pallet" => Fonts::FontPallet,
//         "tiny" => Fonts::FontTiny,
//         _ => Fonts::FontBlock,
//     }
// }
//
