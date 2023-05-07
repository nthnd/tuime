use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Colors {
    /// Uses the system font defined by your console
    System,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright,
    /// A color that randomizes it's colors from a set of bright candy-like color set.
    Candy,
    // /// `Rgb` allows you to use colors outside the traditional ansi16 color set.
    // /// It's value is the [`Rgb`] enum that has a single value called `Val`.
    // Rgb(Rgb),
}

impl From<Colors> for cfonts::Colors {
    fn from(c: Colors) -> Self {
        match c {
            Colors::System => cfonts::Colors::System,
            Colors::Black => cfonts::Colors::Black,
            Colors::Red => cfonts::Colors::Red,
            Colors::Green => cfonts::Colors::Green,
            Colors::Yellow => cfonts::Colors::Yellow,
            Colors::Blue => cfonts::Colors::Blue,
            Colors::Magenta => cfonts::Colors::Magenta,
            Colors::Cyan => cfonts::Colors::Cyan,
            Colors::White => cfonts::Colors::White,
            Colors::Gray => cfonts::Colors::Gray,
            Colors::RedBright => cfonts::Colors::RedBright,
            Colors::GreenBright => cfonts::Colors::GreenBright,
            Colors::YellowBright => cfonts::Colors::YellowBright,
            Colors::BlueBright => cfonts::Colors::BlueBright,
            Colors::MagentaBright => cfonts::Colors::MagentaBright,
            Colors::CyanBright => cfonts::Colors::CyanBright,
            Colors::WhiteBright => cfonts::Colors::WhiteBright,
            Colors::Candy => cfonts::Colors::Candy,
        }
    }
}

/// Colors vector new type wrapper
pub struct ColorsVecNTWrapper(pub Vec<Colors>);

impl From<ColorsVecNTWrapper> for Vec<cfonts::Colors> {
    fn from(c: ColorsVecNTWrapper) -> Self {
        c.0.iter().map(|c| (*c).into()).collect()
    }
}
