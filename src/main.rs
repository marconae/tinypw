mod password;
mod password_test;

use crate::password::{CharacterMode, RandomPassword, DEFAULT_LENGTH};
use arboard::Clipboard;
use clap::Parser;
use std::iter;
use std::process::ExitCode;

#[derive(Debug, Parser, Clone)]
#[command(name = "tinypw", about = "Yet another tiny CLI tool to generate passwords")]
pub struct Args {
    /// Set the password length
    #[arg(short = 'l', long = "length")]
    pub length: Option<usize>,

    /// Copy password to clipboard
    #[arg(short = 'c', long = "clipboard", default_value_t = false)]
    pub to_clipboard: bool,

    /// Mode: include u=uppercase l=lowercase s=symbols n=numbers e=exclude similars
    #[arg(short = 'm', long = "mode", default_value = "ulnse")]
    pub mode: String,

    /// Extra chars to add to the base set of chars
    #[arg(short = 'e', long = "extra", default_value = "")]
    pub extra_chars: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let pw = get_random_password(&args);
    let pw_str = pw.generate();

    println!("Password: {}", pw_str);
    println!("{}", render_strength_bar(&pw_str));

    // Optionally copy to clipboard first to avoid racing with any later prints
    if args.to_clipboard {
        match set_clipboard(&pw_str) {
            Ok(()) => println!("Password copied to clipboard."),
            Err(err) => eprintln!("Warning: failed to copy to clipboard: {}", err),
        }
    }

    ExitCode::SUCCESS
}

fn set_clipboard(s: &str) -> Result<(), String> {
    let mut cb = Clipboard::new().map_err(|e| format!("{}", e))?;
    cb.set_text(s.to_string()).map_err(|e| format!("{}", e))
}

fn get_random_password(args: &Args) -> RandomPassword {
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
        .extra_chars(args.extra_chars.clone())
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

fn render_strength_bar(pw_str: &str) -> String {
    let entropy_bits = password::entropy_bits(&pw_str);

    let cap = 90.0;
    let pct = (entropy_bits / cap).clamp(0.0, 1.0);
    let width = 24usize;


    let count_filled_blocks = (pct * width as f64).round() as usize;
    let count_empty_blocks = width - count_filled_blocks;

    let filled_block = '█';
    let empty_block = '░';

    let label = password::strength_label(entropy_bits);
    let emoji = strength_emoji(label);

    let color = strength_color(label);
    let reset = "\x1b[0m";

    let bar: String = iter::repeat_n(filled_block, count_filled_blocks)
        .chain(iter::repeat_n(empty_block, count_empty_blocks))
        .collect();

    format!("{color}[{bar}] {reset}{:>5.1}% {label} {}", pct * 100.0, emoji)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::password::LETTERS_UPPER;

    impl Args {
        fn new(length: Option<usize>
               , to_clipboard: bool
               , mode: impl Into<String>
               , extra_chars: impl Into<String>) -> Self {
            Self { length, to_clipboard, mode: mode.into(), extra_chars: extra_chars.into() }
        }
    }

    #[test]
    fn test_get_random_password() {
        // Programmatic construction
        let args = Args::new(Some(4), false, "ulnse", "");
        let rnd_pw = get_random_password(&args);
        assert_eq!(rnd_pw.length, 4);

        // When mode contains 'u' and 'l', base_string includes both cases; check uppercase present among base.
        assert!(rnd_pw.base_string.chars().any(|c| LETTERS_UPPER.contains(c)));
    }

    #[test]
    fn test_strength_bar() {
        let actual = render_strength_bar("4kVRwqf73dS*Iu7W");
        assert!(actual.contains("strong"));
        assert!(actual.contains("69.5%"));
    }

    #[test]
    #[ignore]
    fn test_set_clipboard() {
        let s = "test";

        match set_clipboard(&s) {
            Ok(()) => assert!(true),
            Err(err) => {
                eprintln!("Failed to set clipboard: {}", err);
                assert!(false)
            }
        }

        match Clipboard::new().unwrap().get_text() {
            Ok(text) => assert_eq!(text, s.to_string()),
            Err(_) => assert!(false),
        }
    }
}