mod password;
mod password_test;

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

    /// Mode: include u=uppercase l=lowercase s=symbols n=numbers
    /// Default mode is: ulns
    #[arg(short = 'm', long = "mode", default_value = "ulnse")]
    mode: String,

    /// Extra chars to add to the base set of chars
    #[arg(short = 'e', long = "extra", default_value = "")]
    extra_chars: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let pw = get_random_password(args);
    let pw_str = pw.generate();
    let strength = password::entropy_bits(&pw_str);

    println!("Password: {}", pw_str);
    println!("{}", render_strength_bar(strength));
    println!("Entropy: {:.2} bits", strength);

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

// Add this alongside your password helpers

fn strength_color(label: &str) -> &'static str {
    match label {
        "weak" => "\x1b[31m",  // red
        "fair" => "\x1b[33m",  // yellow
        "good" => "\x1b[36m",  // cyan
        _ => "\x1b[32m",       // green
    }
}

fn strength_emoji(label: &str) -> &'static str {
    match label {
        "weak" => "😬",
        "fair" => "😐",
        "good" => "🙂",
        _ => "😎",
    }
}

/// Render a colored progress bar based on entropy (bits).
/// Bar caps at 80 bits for visualization.
fn render_strength_bar(entropy_bits: f64) -> String {
    let cap = 90.0;
    let pct = (entropy_bits / cap).clamp(0.0, 1.0);
    let width = 24usize;
    let filled = (pct * width as f64).round() as usize;

    let filled_block = '█';
    let empty_block = '░';

    let label = password::strength_label(entropy_bits);
    let color = strength_color(label);
    let reset = "\x1b[0m";

    let bar: String = std::iter::repeat(filled_block).take(filled)
        .chain(std::iter::repeat(empty_block).take(width - filled))
        .collect();

    format!("{color}[{bar}] {reset}{:>5.1}% {label} {}", pct * 100.0, strength_emoji(label))
}