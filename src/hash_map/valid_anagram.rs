use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false
    }

    let mut map = HashMap::new();
    for i in s.as_bytes() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    for i in t.as_bytes() {
        if let Some(count) = map.get_mut(i) {
            if *count == 0 {
                return false
            } else {
                *count -= 1
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true);
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}