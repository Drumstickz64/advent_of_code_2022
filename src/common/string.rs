pub fn ascii_char_at(s: &str, index: usize) -> char {
    if !s.is_ascii() {
        panic!("String is not a valid ASCII string");
    }
    s.as_bytes()[index] as char
}

pub fn alphabet_score(ch: char) -> u32 {
    let char_code = ch as u32;
    let is_capital = char_code <= 90;
    if is_capital {
        char_code - 38
    } else {
        char_code - 96
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphbet_score() {
        assert_eq!(alphabet_score('a'), 1);
        assert_eq!(alphabet_score('z'), 26);
        assert_eq!(alphabet_score('A'), 27);
        assert_eq!(alphabet_score('Z'), 52);
    }
}
