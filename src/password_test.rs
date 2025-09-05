#[cfg(test)]
mod tests {
    use crate::password;
    use crate::password::CharacterMode::{Lower, Upper};
    use crate::password::RandomPassword;
    use password::{entropy_bits, strength_label};

    #[test]
    fn test_generate() {
        let expected_length = 20;

        let pw = RandomPassword::builder()
            .length(expected_length)
            .build();

        assert_eq!(pw.length, expected_length);

        let pw_gen_1 = pw.generate();
        assert_eq!(pw_gen_1.len(), expected_length);

        let pw_gen_2 = pw.to_string();
        assert_eq!(pw_gen_2.len(), expected_length);
    }

    #[test]
    fn test_default_values() {
        let pw = RandomPassword::builder().build();

        assert_eq!(pw.length, password::DEFAULT_LENGTH);
        assert!(contains_all(&pw.base_string, password::LETTERS_UPPER));
        assert!(contains_all(&pw.base_string, password::LETTERS_LOWER));
        assert!(contains_all(&pw.base_string, password::NUMBERS));
        assert!(contains_all(&pw.base_string, password::SYMBOLS));
        assert!(contains_all(&pw.base_string, password::SIMILAR_SYMBOLS));
    }

    #[test]
    fn test_exclude_similar_symbols() {
        let pw = RandomPassword::builder()
            .exclude_similar(true)
            .build();

        assert!(contains_none(&pw.base_string, password::SIMILAR_SYMBOLS));
    }

    #[test]
    fn test_character_modes() {
        let pw = RandomPassword::builder()
            .character_mode(Lower)
            .build();

        assert!(contains_none(&pw.base_string, password::LETTERS_UPPER));

        let pw = RandomPassword::builder()
            .character_mode(Upper)
            .build();

        assert!(contains_none(&pw.base_string, password::LETTERS_LOWER));
    }

    #[test]
    fn test_exclude_numbers() {
        let pw = RandomPassword::builder()
            .include_numbers(false)
            .build();

        assert!(contains_none(&pw.base_string, password::NUMBERS));
    }

    #[test]
    fn test_exclude_symbols() {
        let pw = RandomPassword::builder()
            .include_symbols(false)
            .build();

        assert!(contains_none(&pw.base_string, password::SYMBOLS));
    }

    #[test]
    fn test_add_extra_chars() {
        let extra_chars = "~º";
        let pw = RandomPassword::builder()
            .extra_chars(extra_chars.to_string())
            .build();

        assert!(extra_chars.chars().any(|c| pw.base_string.contains(c)));
    }

    // --- labels
    #[test]
    fn strength_label_consistency_with_entropy() {
        // no entropy
        let low_entropy = entropy_bits(&"a");
        assert_eq!(low_entropy, 0.0);

        // "abcd" -> entropy = 8.0 -> weak
        let weak_entropy = entropy_bits(&"abcd");
        assert_eq!(strength_label(weak_entropy), "weak");

        // 10 lowercase letters -> pool ~unique 10 -> 10*log2(10) ≈ 33.22 -> fair
        let fair_entropy = entropy_bits(&"abcdefghij");
        assert_eq!(strength_label(fair_entropy), "fair");

        // 12 lowercase distinct -> 12*log2(12) ≈ 42.78 -> good
        let good_entropy = entropy_bits(&"abcdefghijkl");
        assert_eq!(strength_label(good_entropy), "good");

        // 16 lowercase distinct -> 16*log2(16)=16*4=64 -> strong
        let strong_entropy = entropy_bits(&"abcdefghijklmnop");
        assert_eq!(strength_label(strong_entropy), "strong");
    }

    fn contains_all(a: &str, b: &str) -> bool {
        b.chars().all(|c| a.contains(c))
    }

    fn contains_none(a: &str, b: &str) -> bool {
        !contains_all(a, b)
    }
}