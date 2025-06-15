pub(crate) fn extract_digits(s: &str) -> (&str, &str){
    let mut digit_end = 0;
    
    for (idx, c) in s.char_indices() {
        if c.is_ascii_digit() {
            digit_end = idx + 1;
        } else { 
            break;
        }
    }
    
    let digits = &s[..digit_end];
    let remainder = &s[digit_end..];
    (digits, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn extract_one_digits_test() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }
    
    #[test]
    fn extract_multiple_digits_test() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }
}
