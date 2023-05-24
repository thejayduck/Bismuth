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
    ///  - `BgCenter`: Centers the image on the screen without scaling.
    ///  - `BgFill`: Scales the image to fit the screen and preserves aspect ratio.
    ///  - `BgMax`: Scales the image to the maximum size with black borders on one side.
    ///  - `BgScale`: Fills the screen but doesn't preserve the aspect raio.
    ///  - `BgTile`: Tiles the image on the screen.
    #[clap(long, short, value_enum, default_value_t = Modes::BgFill)]
    pub mode: Modes,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Modes {
    BgCenter,
    BgFill,
    BgMax,
    BgScale,
    BgTile,
}
