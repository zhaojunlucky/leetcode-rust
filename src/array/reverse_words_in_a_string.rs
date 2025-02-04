fn reverse_words(s: String) -> String {
    let mut ans: Vec<String> = vec![];

    let mut j = -1;
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == ' ' {
            if j >= 0 {
                let w = &s[j as usize..i];
                ans.insert(0, w.parse().unwrap());
                j = -1;
            }
        } else if j == -1 {
            j = i as i32;
        }
    }
    if j >= 0 {
        let w = &s[j as usize..];
        ans.insert(0, w.parse().unwrap());
    }


    ans.join(" ")
}

fn reverse_words2(s: String) -> String {
    s.split(" ").filter(|&word| !word.is_empty()).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!("h", reverse_words("    h    ".to_string()));
        assert_eq!("h", reverse_words("h".to_string()));
        assert_eq!("blue is sky the", reverse_words("the sky is blue".to_string()));
        assert_eq!("world hello", reverse_words("hello world  ".to_string()));
        assert_eq!("example good a", reverse_words("a good   example".to_string()));

        assert_eq!("h", reverse_words2("    h    ".to_string()));
        assert_eq!("h", reverse_words2("h".to_string()));
        assert_eq!("blue is sky the", reverse_words2("the sky is blue".to_string()));
        assert_eq!("world hello", reverse_words2("hello world  ".to_string()));
        assert_eq!("example good a", reverse_words2("a good   example".to_string()));
    }
}