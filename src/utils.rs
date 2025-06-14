pub(crate) fn extract_digits(s: &str) -> (&str, &str){
    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn extract_digits_test() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }
}
