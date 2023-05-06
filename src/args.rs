use crate::{colors::Colors, fonts::Fonts};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Format the time
    #[arg(long, default_value_t = String::from("%H:%M"))]
    pub format: String,

    /// Supply a color to use for rendering
    ///
    /// If the font supports it you may supply mutliple colors : -c red -c green ...
    /// To see what fonts support multiple colors, see https://github.com/dominikwilkowski/cfonts"
    #[arg(short, long, value_enum, verbatim_doc_comment)]
    pub colors: Vec<Colors>,

    /// Set the font
    /// To see what fonts you can use, go to https://github.com/dominikwilkowski/cfonts"
    #[arg(short, long, value_enum, default_value_t = Fonts::FontBlock, verbatim_doc_comment)]
    pub font: Fonts,

    /// Set a gradient instead of regular colors : -g "#ffaabb" -g "#ee22ff" ..."
    #[arg(short, long, verbatim_doc_comment)]
    pub gradient: Vec<String>,

    /// Set the utc offset
    ///
    /// If this argument is not supplied, we will try to use the local time
    /// Supplied as +/-<secs>. Eg: tuime -u="-3600", tuime -u="+7200"
    #[arg(short, long, verbatim_doc_comment)]
    pub utc_offset: Option<i32>,

    /// Screensaver mode
    #[arg(short, long)]
    pub screensaver: bool,
}
