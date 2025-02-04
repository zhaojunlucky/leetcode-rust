fn length_of_last_word(s: String) -> i32 {
    let mut ans = 0;

    let mut i = s.len() - 1;
    while i > 0 && s.chars().nth(i).unwrap() == ' ' {
        i -= 1;
    }
    while i >= 0 && s.chars().nth(i).unwrap() != ' ' {
        ans += 1;
        if i == 0 {
            break
        }
        i -= 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word(" h ".to_string()), 1);
        assert_eq!(length_of_last_word("hh".to_string()), 2);
        assert_eq!(length_of_last_word("h".to_string()), 1);
    }
}