use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

fn is_email_address(string: &str) -> bool {
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]{2,}$").unwrap();
    email_regex.is_match(string)
}

pub fn match_email_address(string: &str) -> String {
    if is_email_address(string) {
        format!("{string} 📩✅").to_string()
    } else {
        format!("{string} 📩❎").to_string()
    }
}

pub fn alternate_string_case(string: &str) -> String {
    let graphemes = string.graphemes(true).collect::<Vec<&str>>();
    graphemes
        .iter()
        .enumerate()
        .map(|(idx, g)| {
            if idx % 2 == 0 {
                g.to_lowercase()
            } else {
                g.to_uppercase()
            }
        })
        .collect()
}

pub fn is_string_ordered(string: &str) -> String {
    if are_words_ordered(string) {
        format!("Woah, would you look at that: the words in {string} are sorted alphabetically!")
    } else {
        format!("The words in {string} are unsorted. Silly you.")
    }
}

fn are_words_ordered(string: &str) -> bool {
    let words = string
        .unicode_words()
        .collect::<Vec<&str>>()
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<String>>();
    words.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod test {
    use crate::strings::{alternate_string_case, are_words_ordered, is_email_address};

    #[test]
    fn test_words_ordered() {
        assert!(are_words_ordered("deez nuts"))
    }

    #[test]
    fn test_words_ordered_mixed_case() {
        assert!(are_words_ordered("Deez Nuts zigma"))
    }

    #[test]
    fn test_single_word_is_ordered() {
        assert!(are_words_ordered("deez"))
    }

    #[test]
    fn test_no_word_is_ordered() {
        assert!(are_words_ordered(""))
    }

    #[test]
    fn test_words_unordered() {
        assert!(!are_words_ordered("nuts deez"))
    }

    #[test]
    fn test_words_unordered_mixed_case() {
        assert!(!are_words_ordered("Zigma nuts Deez"))
    }

    #[test]
    fn test_actual_email_address() {
        let actual = is_email_address("user@domain.com");
        assert!(actual);
    }

    #[test]
    fn test_email_address_empty() {
        let actual = is_email_address("");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_starts_with_spaces() {
        let actual = is_email_address(" user@domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_ends_with_spaces() {
        let actual = is_email_address("user@domain.com ");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_missing_domain() {
        let actual = is_email_address("user@.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_missing_top_level_domain() {
        let actual = is_email_address("user@domain");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_missing_user() {
        let actual = is_email_address("@domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_spaces_in_domain() {
        let actual = is_email_address("user@ domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_spaces_in_user() {
        let actual = is_email_address("us er@domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_spaces_in_top_level_domain() {
        let actual = is_email_address("user@domain.c om");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_top_level_domain_too_short() {
        let actual = is_email_address("user@domain.c");
        assert!(!actual);
    }

    #[test]
    fn test_alternate_case_basic_string() {
        let string = alternate_string_case("simple");
        assert_eq!(string, "sImPlE");
    }

    #[test]
    fn test_alternate_case_with_spaces() {
        let string = alternate_string_case("hi sir");
        assert_eq!(string, "hI SiR");
    }

    #[test]
    fn test_alternate_case_with_special_characters() {
        let string = alternate_string_case("têstãç");
        assert_eq!(string, "tÊsTãÇ");
    }

    #[test]
    fn test_alternate_case_with_emojis() {
        let string = alternate_string_case("🤓");
        assert_eq!(string, "🤓");
    }
}
