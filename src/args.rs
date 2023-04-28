use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Format the time
    #[arg(long, default_value_t = String::from("%H:%M"))]
    pub format: String,

    #[arg(
        short,
        long,
        help = 
r"Supply a color to use for rendering
If the font supports it you may supply mutliple colors : -c red -c green ...
To see what fonts support multiple colors, see https://github.com/dominikwilkowski/cfonts"
    )]
    pub colors: Vec<String>,

    #[arg(
        short, 
        long, 
        default_value_t = String::from("Block"),
        help = 
r"Set Font
To see what fonts you can use, go to https://github.com/dominikwilkowski/cfonts"
    )]
    pub font: String,

    #[arg(short, long, help=
r#"Set a gradient
use a gradient instead of regular colors : -g "\#ffaabb" -g "\#ee22ff" ..."#
    )]
    pub gradient: Vec<String>,
}
