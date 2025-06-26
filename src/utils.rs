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

pub(crate) fn extract_ident(s: &str) -> (&str, &str){
    // take_while(|c: char| c.is_ascii_alphanumeric(), s)
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        take_while(|c: char| c.is_ascii_alphanumeric(), s)
    } else {
        (s, "")
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(starting_text) {
        &s[starting_text.len()..]
    } else { 
        panic!("expected {}, got {}", starting_text, s);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_word(){
        assert_eq!(tag("foo", "foo a"), " a");
        assert_eq!(tag("let", "let a"), " a");
    }
    
    #[test]
    fn extract_ident_starts_with_num_error(){
        assert_eq!(extract_ident("123"), ("123", ""));
    }

    #[test]
    fn extract_alphabetic_ident(){
        assert_eq!(extract_ident("abcd stop"), (" stop", "abcd"));
    }

    #[test]
    fn extract_alphabetic_func(){
        assert_eq!(extract_ident("function01()"), ("()", "function01"));
    }

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
