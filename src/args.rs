use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Format the time
    #[arg(short, long, default_value_t = String::from("%H:%M"))]
    pub format: String,
}
