mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Cli;
use clap::Parser;
use std::fs;

use crate::args::PngCushionSubcommands;
use crate::png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    /*
    Sample commands:
    - pngme encode ./dice.png ruSt "This is a secret message!
    - pngme decode ./dice.png ruSt
    - pngme remove ./dice.png ruSt
    - pngme print ./dice.png
    */

    match dbg!(cli).command {
        PngCushionSubcommands::Encode(args) => {
            let f = fs::read(args.in_file)?;
            let p = Png::try_from(f.as_slice())?;
            println!("{}", p);
        }
        PngCushionSubcommands::Decode(args) => {
            let f = fs::read(args.in_file)?;
        }
        PngCushionSubcommands::Print(args) => {
            let f = fs::read(args.in_file)?;
        }
        PngCushionSubcommands::Remove(args) => {
            let f = fs::read(args.in_file)?;
        }
    }

    Ok(())
}
