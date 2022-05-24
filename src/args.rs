use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "PNG Cushion")]
#[clap(author = "Benjamin Liden <benjamin.c.liden@gmail.com>")]
#[clap(version = "0.1")]
#[clap(propagate_version = true)]
#[clap(about = "Encode and decode secret messages in PNG files", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: PngCushionSubcommands,
}

/*
Sample commands:
- pngme encode ./dice.png ruSt "This is a secret message!
- pngme decode ./dice.png ruSt
- pngme remove ./dice.png ruSt
- pngme print ./dice.png
*/

#[derive(Subcommand, Debug)]
pub enum PngCushionSubcommands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    #[clap(help = "PNG file to read from", parse(from_os_str))]
    pub in_file: PathBuf,

    #[clap(help = "What chunk key to add your message to")]
    pub chunk_type: String,

    #[clap(help = "Secret message to pass into the PNG file")]
    pub message: String,

    #[clap(help = "PNG file to print out", parse(from_os_str))]
    pub out_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    #[clap(help = "PNG file to read from", parse(from_os_str))]
    pub in_file: PathBuf,

    #[clap(help = "What chunk key to read a message from")]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    #[clap(help = "PNG file to read from", parse(from_os_str))]
    pub in_file: PathBuf,

    #[clap(help = "What chunk key to remove from the PNG")]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    #[clap(help = "PNG file to read from", parse(from_os_str))]
    pub in_file: PathBuf,
}
