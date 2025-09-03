#[cfg(test)]
mod tests {
    use crate::password;
    use crate::password::CharacterMode::{Lower, Upper};
    use crate::password::RandomPassword;

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
        let extra_chars = ['~'];
        let pw = RandomPassword::builder()
            .extra_chars(extra_chars.to_vec())
            .build();

        assert!(extra_chars.iter().all(|c| pw.base_string.contains(*c)))
    }

    fn contains_all(a: &str, b: &str) -> bool {
        b.chars().all(|c| a.contains(c))
    }

    fn contains_none(a: &str, b: &str) -> bool {
        !contains_all(a, b)
    }

}