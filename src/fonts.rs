use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Fonts {
	FontConsole,
	FontBlock,
	FontSimpleBlock,
	FontSimple,
	Font3d,
	FontSimple3d,
	FontChrome,
	FontHuge,
	FontShade,
	FontSlick,
	FontGrid,
	FontPallet,
	FontTiny,
}

impl From<Fonts> for cfonts::Fonts {
    fn from(f: Fonts) -> Self {
        match f {
            Fonts::FontConsole => cfonts::Fonts::FontConsole,
            Fonts::FontBlock => cfonts::Fonts::FontBlock,
            Fonts::FontSimpleBlock => cfonts::Fonts::FontSimpleBlock,
            Fonts::FontSimple => cfonts::Fonts::FontSimple,
            Fonts::Font3d => cfonts::Fonts::Font3d,
            Fonts::FontSimple3d => cfonts::Fonts::FontSimple3d,
            Fonts::FontChrome => cfonts::Fonts::FontChrome,
            Fonts::FontHuge => cfonts::Fonts::FontHuge,
            Fonts::FontShade => cfonts::Fonts::FontShade,
            Fonts::FontSlick => cfonts::Fonts::FontSlick,
            Fonts::FontGrid => cfonts::Fonts::FontGrid,
            Fonts::FontPallet => cfonts::Fonts::FontPallet,
            Fonts::FontTiny => cfonts::Fonts::FontTiny,
        }
    }
}
