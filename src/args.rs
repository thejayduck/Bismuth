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
    ///  - `Center`: Centers the image on the screen without scaling.
    ///  - `Fill`: Scales the image to fit the screen and preserves aspect ratio.
    ///  - `Max`: Scales the image to the maximum size with black borders on one side.
    ///  - `Scale`: Fills the screen but doesn't preserve the aspect raio.
    ///  - `Tile`: Tiles the image on the screen.
    #[clap(long, short, value_enum, default_value_t = Modes::Fill)]
    pub mode: Modes,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Modes {
    Center,
    Fill,
    Max,
    Scale,
    Tile,
}
