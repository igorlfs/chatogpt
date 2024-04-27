pub fn caesar_cipher(string: &str, mut shift: u32) -> String {
    shift = shift % 26;
    let encrypted_text = String::from(string);
    encrypted_text
        .chars()
        .map(|c| {
            if !c.is_alphabetic() {
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

#[cfg(test)]
mod test {
    use super::caesar_cipher;

    #[test]
    fn test_caesar_cipher_empty_string() {
        let encrypted_string = caesar_cipher("", 5);
        assert_eq!(encrypted_string, "");
    }

    #[test]
    fn test_caesar_cipher_with_size_one_string(){
        let encrypted_string = caesar_cipher("A", 1);
        assert_eq!(encrypted_string, "B");
    }

    #[test]
    fn test_caesar_cipher_with_shift_0(){
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 0);
        assert_eq!(encrypted_string, "Let's kill the king Aegon and blame the jester!");
    }

    #[test]
    fn test_caesar_cipher_with_shift_26(){
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 0);
        assert_eq!(encrypted_string, "Let's kill the king Aegon and blame the jester!");
    }

    #[test]
    fn test_caesar_cipher_with_shift_5(){
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 5);
        assert_eq!(encrypted_string, "Qjy'x pnqq ymj pnsl Fjlts fsi gqfrj ymj ojxyjw!");
    }

    #[test]
    fn test_caesar_cipher_with_shift_25(){
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 25);
        assert_eq!(encrypted_string, "Kds'r jhkk sgd jhmf Zdfnm zmc akzld sgd idrsdq!");
    }

    #[test]
    fn test_caesar_cipher_with_not_alphabetic_chars(){
        let encrypted_string = caesar_cipher("*&%..:077%", 5);
        assert_eq!(encrypted_string, "*&%..:077%");
    }

    #[test]
    fn test_caesar_cipher_with_all_chars_uppercase(){
        let encrypted_string = caesar_cipher("HI MOM", 5);
        assert_eq!(encrypted_string, "MN RTR");
    }

    #[test]
    fn test_caesar_cipher_with_all_chars_lowercase(){
        let encrypted_string = caesar_cipher("hi mom", 5);
        assert_eq!(encrypted_string, "mn rtr");
    }
}
