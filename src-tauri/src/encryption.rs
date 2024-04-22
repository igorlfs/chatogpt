pub fn caesar_cipher(string: &str, shift: u32) -> String {
    let encrypted_text = String::from(string);
    encrypted_text
        .chars()
        .map(|c| {
            if (!c.is_alphabetic()) {
                c
            } else {
                let base = if c.is_lowercase() {
                    'a' as u32
                } else {
                    'A' as u32
                };
                char::from_u32((((c as u32 - base) as u32) + shift) % 26 + base).unwrap()
            }
        })
        .collect()
}

#[cfg(text)]
mod test {
    use super::caesar_cipher;

    #[test]
    fn test_caesar_cipher_empty_string() {
        let encrypted_string = caesar_cipher("");
        assert_eq!(encrypted_string, "");
    }
}
