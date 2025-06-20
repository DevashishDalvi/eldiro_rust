pub(crate) fn extract_digits(s: &str) -> (&str, &str){
    take_while(|c: char| c.is_ascii_digit(), s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c: char| c.is_whitespace(), s)
}

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str){
    let extract_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or(s.len());

    let extracted = &s[..extract_end];
    let remainder = &s[extract_end..];
    (remainder, extracted)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn extract_space(){
        assert_eq!(extract_whitespace("  1"), ("1", "  "));
    }
    
    #[test]
    fn extract_one_digits_test() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }
    
    #[test]
    fn extract_multiple_digits_test() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_op("+2"), ("2", "+"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("-10"), ("10", "-"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_op("*3"), ("3", "*"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_op("/4"), ("4", "/"));
    }
    
}
