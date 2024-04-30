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

pub fn vigenere_cipher(string: &str, key: &str) -> String {
    let key_size = key.chars().count();
    if key_size == 0 {
        return string.to_string();
    }
    let uppercase_key = key.to_uppercase();
    let encrypted_text = String::from(string);
    let mut idx = 0;
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
                idx = idx + 1;
                let shift =
                    uppercase_key.chars().nth((idx - 1) % key_size).unwrap() as u32 - ('A' as u32);
                char::from_u32((c as u32 - base + shift) % 26 + base).unwrap()
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::caesar_cipher;
    use super::vigenere_cipher;

    #[test]
    fn test_caesar_cipher_empty_string() {
        let encrypted_string = caesar_cipher("", 5);
        assert_eq!(encrypted_string, "");
    }

    #[test]
    fn test_caesar_cipher_with_size_one_string() {
        let encrypted_string = caesar_cipher("A", 1);
        assert_eq!(encrypted_string, "B");
    }

    #[test]
    fn test_caesar_cipher_with_shift_0() {
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 0);
        assert_eq!(
            encrypted_string,
            "Let's kill the king Aegon and blame the jester!"
        );
    }

    #[test]
    fn test_caesar_cipher_with_shift_26() {
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 0);
        assert_eq!(
            encrypted_string,
            "Let's kill the king Aegon and blame the jester!"
        );
    }

    #[test]
    fn test_caesar_cipher_with_shift_5() {
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 5);
        assert_eq!(
            encrypted_string,
            "Qjy'x pnqq ymj pnsl Fjlts fsi gqfrj ymj ojxyjw!"
        );
    }

    #[test]
    fn test_caesar_cipher_with_shift_25() {
        let encrypted_string = caesar_cipher("Let's kill the king Aegon and blame the jester!", 25);
        assert_eq!(
            encrypted_string,
            "Kds'r jhkk sgd jhmf Zdfnm zmc akzld sgd idrsdq!"
        );
    }

    #[test]
    fn test_caesar_cipher_with_not_alphabetic_chars() {
        let encrypted_string = caesar_cipher("*&%..:077%", 5);
        assert_eq!(encrypted_string, "*&%..:077%");
    }

    #[test]
    fn test_caesar_cipher_with_all_chars_uppercase() {
        let encrypted_string = caesar_cipher("HI MOM", 5);
        assert_eq!(encrypted_string, "MN RTR");
    }

    #[test]
    fn test_caesar_cipher_with_all_chars_lowercase() {
        let encrypted_string = caesar_cipher("hi mom", 5);
        assert_eq!(encrypted_string, "mn rtr");
    }

    #[test]
    fn test_vigenere_cipher_with_empty_key() {
        let encrypted_string =
            vigenere_cipher("Let's kill the king Aegon and blame the jester!", "");
        assert_eq!(
            encrypted_string,
            "Let's kill the king Aegon and blame the jester!"
        );
    }

    #[test]
    fn test_vigenere_cipher_with_size_1_key() {
        let encrypted_string =
            vigenere_cipher("Let's kill the king Aegon and blame the jester!", "w");
        assert_eq!(
            encrypted_string,
            "Hap'o gehh pda gejc Wackj wjz xhwia pda faopan!"
        );
    }

    #[test]
    fn test_vigenere_cipher_with_size_5_key() {
        let encrypted_string =
            vigenere_cipher("Let's kill the king Aegon and blame the jester!", "syrax");
        assert_eq!(
            encrypted_string,
            "Dck's hajc tew iznd Scxok slu biskv tew hvsqwp!"
        );
    }

    #[test]
    fn test_vigenere_cipher_with_long_key() {
        let encrypted_string = vigenere_cipher(
            "Let's kill the king Aegon and blame the jester!",
            "syraxvermithorvhagarcaraxestessarionmeleys",
        );
        assert_eq!(
            encrypted_string,
            "Dck's hdpc fpx rweb Hemoe cnu bieex xzw jvahrd!"
        );
    }

    #[test]
    fn test_vigenere_cipher_with_key_longer_than_text() {
        let encrypted_string = vigenere_cipher("Aegon", "areallylongkey");
        assert_eq!(encrypted_string, "Avkoy");
    }

    #[test]
    fn test_vigenere_cipher_with_empty_string() {
        let encrypted_string = vigenere_cipher("", "syrax");
        assert_eq!(encrypted_string, "");
    }

    #[test]
    fn test_vigenere_cipher_with_not_alphabetic_chars() {
        let encrypted_string = vigenere_cipher("!@##!@.:??^]]", "syrax");
        assert_eq!(encrypted_string, "!@##!@.:??^]]");
    }

    #[test]
    fn test_vigenere_cipher_with_all_chars_lowercase() {
        let encrypted_string = vigenere_cipher("i want to kill king aegon", "syrax");
        assert_eq!(encrypted_string, "a urnq lm biid iznd scxok");
    }

    #[test]
    fn test_vigenere_cipher_with_all_chars_uppercase() {
        let encrypted_string = vigenere_cipher("I WANT TO KILL KING VISERYS", "syrax");
        assert_eq!(encrypted_string, "A URNQ LM BIID IZND NGJEOQQ");
    }
}
