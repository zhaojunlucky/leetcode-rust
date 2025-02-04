fn str_str(haystack: String, needle: String) -> i32 {
    let i = haystack.find(needle.as_str()).unwrap_or(haystack.len()) as i32;

    if i == haystack.len() as i32 {
        -1
    } else {
        i
    }

}

fn str_str2(haystack: String, needle: String) -> i32 {
    if needle.len() > haystack.len() {
        return -1;
    }
    let max = haystack.len() - needle.len();

    for i in 0..=max {
        if haystack[i..i+needle.len()] == needle {
            return i as i32
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
        assert_eq!(str_str2("sadbutsad".to_string(), "sad".to_string()), 0);
        assert_eq!(str_str2("leetcode".to_string(), "leeto".to_string()), -1);
    }
}
