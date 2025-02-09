use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> i32 {
    let ss = s.as_bytes();
    let mut window: HashSet<u8> = HashSet::new();
    let mut left = 0;
    let mut ans = 0;

    for i in 0..ss.len() {
        while window.contains(&ss[i]) {
            window.remove(&ss[left]);
            left += 1;
        }
        window.insert(ss[i]);
        ans = ans.max(window.len());

    }

    ans as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}