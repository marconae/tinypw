mod password;
mod password_test;
mod faulty;

use std::process::ExitCode;
use crate::password::{CharacterMode, RandomPassword, DEFAULT_LENGTH};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "tinypw", about = "Yet another tiny CLI tool to generate passwords")]
struct Args {
    /// Set the password length
    #[arg(short = 'l', long = "length")]
    length: Option<usize>,

    /// Copy password to clipboard
    #[arg(short = 'c', long = "clipboard", default_value_t = false)]
    to_clipboard: bool,

    /// Mode: include u=uppercase l=lowercase s=symbols n=numbers e=exclude similars
    #[arg(short = 'm', long = "mode", default_value = "ulnse")]
    mode: String,

    /// Extra chars to add to the base set of chars
    #[arg(short = 'e', long = "extra", default_value = "")]
    extra_chars: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let pw = get_random_password(args);
    println!("{}", pw.generate());

    ExitCode::SUCCESS
}

fn get_random_password(args: Args) -> RandomPassword {
    let include_numbers = args.mode.contains('n');
    let include_symbols = args.mode.contains('s');
    let exclude_similars = args.mode.contains('e');

    let character_mode = match (
        args.mode.contains('u'),
        args.mode.contains('l'),
    ) {
        (true, true) => CharacterMode::LowerUpper,
        (true, false) => CharacterMode::Upper,
        (false, true) => CharacterMode::Lower,
        (false, false) => password::DEFAULT_CHARACTER_MODE,
    };

    RandomPassword::builder()
        .length(args.length.unwrap_or(DEFAULT_LENGTH))
        .character_mode(character_mode)
        .include_numbers(include_numbers)
        .include_symbols(include_symbols)
        .exclude_similar(exclude_similars)
        .build()
}