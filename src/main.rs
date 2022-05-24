mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Cli;
use clap::Parser;
use std::fs;
use std::str::FromStr;

use crate::args::PngCushionSubcommands;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
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
            let f = fs::read(args.in_file.clone())?;
            let mut p = Png::try_from(f.as_slice())?;
            if p.chunk_by_type(args.chunk_type.as_str()).is_some() {
                return Err("[❌] Chunk type already encoded".into());
            }

            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let chunk = Chunk::new(chunk_type, args.message.into_bytes());
            p.append_chunk(chunk);

            if let Some(file) = args.out_file {
                println!("[ℹ] Writing to specified outfile: {:?}", file);
                fs::write(file, p.as_bytes())?;
            } else {
                println!("[ℹ] Overwriting to same file ({:?})...", args.in_file);
                fs::write(args.in_file, p.as_bytes())?;
            }
        }
        PngCushionSubcommands::Decode(args) => {
            let f = fs::read(args.in_file.clone())?;
            let p = Png::try_from(f.as_slice())?;

            println!(
                "Searching for chunks of the type \"{}\" within {:?}...",
                args.chunk_type, args.in_file
            );
            if let Some(chunk) = p.chunk_by_type(&args.chunk_type) {
                println!(
                    "[✅] Found message within chunk \"{}\" with message \"{}\"",
                    args.chunk_type,
                    chunk.data_as_string()?
                );
            } else {
                println!(
                    "[❌] No message found for chunk type \"{}\"",
                    &args.chunk_type
                );
            }
        }
        PngCushionSubcommands::Print(args) => {
            let f = fs::read(args.in_file)?;
            let p = Png::try_from(f.as_slice())?;
            println!("{}", p);
        }
        PngCushionSubcommands::Remove(args) => {
            let f = fs::read(args.in_file.clone())?;
            let mut p = Png::try_from(f.as_slice())?;

            println!(
                "[ℹ] Searching for chunks of the type \"{}\" within {:?}...",
                args.chunk_type, args.in_file
            );
            if p.chunk_by_type(&args.chunk_type).is_some() {
                let chunk = p.remove_chunk(&args.chunk_type)?;
                println!(
                    "[✅] Removed message in \"{:?}\": \"{}\"",
                    args.in_file,
                    chunk.data_as_string()?
                );
                println!("[ℹ] Overwriting file ({:?})...", args.in_file);
                fs::write(args.in_file, p.as_bytes())?;
            } else {
                println!(
                    "[❌] No message found within \"{:?}\" for chunk type \"{}\"",
                    args.in_file, &args.chunk_type
                );
            }
        }
    };

    Ok(())
}
