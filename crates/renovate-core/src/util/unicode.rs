pub fn is_unicode_letter(c: char) -> bool {
    c.is_alphabetic()
}

pub fn strip_emoji(input: &str) -> String {
    input
        .chars()
        .filter(|c| {
            let cp = *c as u32;
            !((0x1F600..=0x1F64F).contains(&cp)
                || (0x1F300..=0x1F5FF).contains(&cp)
                || (0x1F680..=0x1F6FF).contains(&cp)
                || (0x1F1E0..=0x1F1FF).contains(&cp)
                || (0x2600..=0x26FF).contains(&cp)
                || (0x2700..=0x27BF).contains(&cp)
                || (0xFE00..=0xFE0F).contains(&cp)
                || (0x1F900..=0x1F9FF).contains(&cp)
                || (0x1FA00..=0x1FA6F).contains(&cp)
                || (0x1FA70..=0x1FAFF).contains(&cp)
                || cp == 0x200D
                || cp == 0x20E3)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_unicode_letter_ascii() {
        assert!(is_unicode_letter('a'));
        assert!(is_unicode_letter('Z'));
    }

    #[test]
    fn is_unicode_letter_unicode() {
        assert!(is_unicode_letter('\u{4e2d}'));
        assert!(is_unicode_letter('\u{00e9}'));
    }

    #[test]
    fn is_unicode_letter_digit_false() {
        assert!(!is_unicode_letter('1'));
    }

    #[test]
    fn is_unicode_letter_space_false() {
        assert!(!is_unicode_letter(' '));
    }

    #[test]
    fn is_unicode_letter_punct_false() {
        assert!(!is_unicode_letter('.'));
    }

    #[test]
    fn strip_emoji_removes_emoji() {
        assert_eq!(strip_emoji("Hello \u{1f600} World"), "Hello  World");
    }

    #[test]
    fn strip_emoji_no_emoji() {
        assert_eq!(strip_emoji("Hello World"), "Hello World");
    }

    #[test]
    fn strip_emoji_empty() {
        assert_eq!(strip_emoji(""), "");
    }

    #[test]
    fn strip_emoji_multiple() {
        let result = strip_emoji("\u{1f600}\u{1f680}test");
        assert_eq!(result, "test");
    }
}
