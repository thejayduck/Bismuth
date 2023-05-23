use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    /// Disables notifications.
    #[clap(long, short, action)]
    pub silent: bool,

    /// Specifies the scaling options for Feh.
    ///
    /// Available modes:
    ///  - `BgTile`: Tiles the image on the screen.
    ///  - `BgCenter`: Centers the image on the screen without scaling.
    ///  - `BgMax`: Maximizes image while maintaining aspect ratio.
    ///  - `BgFill`: Scales the image to fit the screen.
    #[clap(long, short, value_enum, default_value_t = Modes::BgFill)]
    pub mode: Modes,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Modes {
    BgTile,
    BgCenter,
    BgMax,
    BgFill,
}
