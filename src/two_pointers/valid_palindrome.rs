fn is_palindrome(s: String) -> bool {

    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if !s.as_bytes()[i].is_ascii_alphanumeric() {
            i += 1;
            continue;
        }
        if !s.as_bytes()[j].is_ascii_alphanumeric() {
            j -= 1;
            continue;
        }
        if s.as_bytes()[i].to_ascii_lowercase() != s.as_bytes()[j].to_ascii_lowercase() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_palindrome(" ".to_string()), true);
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }
}