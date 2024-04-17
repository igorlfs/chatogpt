use unicode_segmentation::UnicodeSegmentation;

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

#[cfg(test)]
mod test {
    use super::alternate_string_case;

    #[test]
    fn test_happy_string() {
        let string = alternate_string_case("simple");
        assert_eq!(string, "sImPlE");
    }

    #[test]
    fn test_string_with_spaces() {
        let string = alternate_string_case("hi sir");
        assert_eq!(string, "hI SiR");
    }

    #[test]
    fn test_string_with_special_characters() {
        let string = alternate_string_case("têstãç");
        assert_eq!(string, "tÊsTãÇ");
    }

    #[test]
    fn test_string_with_emojis() {
        let string = alternate_string_case("🤓");
        assert_eq!(string, "🤓");
    }
}
