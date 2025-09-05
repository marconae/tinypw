use crate::password::CharacterMode::LowerUpper;
use rand::distr::Uniform;
use rand::Rng;

pub const LETTERS_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
pub const LETTERS_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = "!#$&()*+/";
pub const SIMILAR_SYMBOLS: &str = "il1o0O";
pub const DEFAULT_LENGTH: usize = 16;
pub const DEFAULT_CHARACTER_MODE: CharacterMode = LowerUpper;
pub const DEFAULT_INCLUDE_NUMBERS: bool = true;
pub const DEFAULT_INCLUDE_SYMBOLS: bool = true;
pub const DEFAULT_EXCLUDE_SIMILAR: bool = false;

pub enum CharacterMode {
    Lower,
    Upper,
    LowerUpper,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RandomPassword {
    pub length: usize,
    pub base_string: String,
}

impl RandomPassword {
    pub fn builder() -> RandomPasswordBuilder {
        RandomPasswordBuilder::default()
    }

    pub fn generate(&self) -> String {
        if self.length == 0 || self.base_string.is_empty() {
            return String::new();
        }

        let mut rng = rand::rng();
        let base_string_length = self.base_string.len();
        let dist = Uniform::try_from(0..base_string_length).expect("invalid uniform range");
        let bytes = self.base_string.as_bytes();

        let mut result = String::with_capacity(self.length);

        for _ in 0..self.length {
            let idx: usize = rng.sample(dist);
            result.push(bytes[idx] as char);
        }

        result
    }
}

/// Calculate Shannon entropy (in bits) for passwords generated with this configuration.
/// Assumes independent uniform selection over the configured character set.
pub fn entropy_bits(str: &str) -> f64 {
    let length = str.len() as f64;
    let pool_size = unique_chars(str) as f64;
    if length == 0.0 || pool_size <= 1.0 {
        return 0.0;
    }
    length * pool_size.log2()
}

/// A simple qualitative label derived from entropy.
/// Thresholds: <28 weak, <36 fair, <60 good, otherwise strong.
pub fn strength_label(e: f64) -> &'static str {
    if e < 28.0 {
        "weak"
    } else if e < 36.0 {
        "fair"
    } else if e < 60.0 {
        "good"
    } else {
        "strong"
    }
}

fn unique_chars(s: &str) -> usize {
    let mut seen = [false; 256];
    let mut count = 0usize;
    for &b in s.as_bytes() {
        let idx = b as usize;
        if !seen[idx] {
            seen[idx] = true;
            count += 1;
        }
    }
    count
}

impl core::fmt::Display for RandomPassword {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = self.generate();
        f.write_str(&s)
    }
}

pub struct RandomPasswordBuilder {
    length: usize,
    character_mode: CharacterMode,
    include_numbers: bool,
    include_symbols: bool,
    exclude_similar: bool,
    extra_chars: Vec<char>,
}

impl Default for RandomPasswordBuilder {
    fn default() -> Self {
        Self {
            length: DEFAULT_LENGTH,
            character_mode: DEFAULT_CHARACTER_MODE,
            include_numbers: DEFAULT_INCLUDE_NUMBERS,
            include_symbols: DEFAULT_INCLUDE_SYMBOLS,
            exclude_similar: DEFAULT_EXCLUDE_SIMILAR,
            extra_chars: Vec::new(),
        }
    }
}

impl RandomPasswordBuilder {
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn character_mode(mut self, mode: CharacterMode) -> Self {
        self.character_mode = mode;
        self
    }

    pub fn include_numbers(mut self, yes: bool) -> Self {
        self.include_numbers = yes;
        self
    }

    pub fn include_symbols(mut self, yes: bool) -> Self {
        self.include_symbols = yes;
        self
    }

    pub fn exclude_similar(mut self, yes: bool) -> Self {
        self.exclude_similar = yes;
        self
    }

    pub fn extra_chars(mut self, chars: Vec<char>) -> Self {
        self.extra_chars = chars;
        self
    }

    fn build_base_string(&self) -> String {
        let mut base_string = String::new();

        match self.character_mode {
            LowerUpper => {
                base_string.push_str(LETTERS_LOWER);
                base_string.push_str(LETTERS_UPPER);
            }
            CharacterMode::Lower => base_string.push_str(LETTERS_LOWER),
            CharacterMode::Upper => base_string.push_str(LETTERS_UPPER),
        }

        if self.include_numbers {
            base_string.push_str(NUMBERS);
        }
        if self.include_symbols {
            base_string.push_str(SYMBOLS);
        }

        if self.exclude_similar {
            base_string.retain(|c| !SIMILAR_SYMBOLS.contains(c));
        }

        self.extra_chars.iter().for_each(|c| base_string.push(*c));
        base_string
    }

    pub fn build(self) -> RandomPassword {
        let base_string = self.build_base_string();
        RandomPassword {
            length: self.length,
            base_string,
        }
    }
}


