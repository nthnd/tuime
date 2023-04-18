use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Format the time
    #[arg(long, default_value_t = String::from("%H:%M"))]
    pub format: String,

    /// Set Colors
    #[arg(short, long)]
    pub colors: Vec<String>,

    /// Set Font
    #[arg(short, long, default_value_t = String::from("Block"))]
    pub font: String,

    /// Set a gradient
    #[arg(short, long)]
    pub gradient: Vec<String>,
}
