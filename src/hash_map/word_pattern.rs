use std::collections::{HashMap, HashSet};

fn word_pattern(pattern: String, s: String) -> bool {
    let mut ss = s.split_ascii_whitespace().collect::<Vec<_>>();
    if ss.len() != pattern.len() {
        return false;
    }
    let mut map = HashMap::new();
    let mut set = HashSet::new();

    let pp = pattern.as_bytes();

    for i in 0..pp.len() {
        if map.contains_key(&pp[i]) {
            if map[&pp[i]] != ss[i] {
                return false;
            }
        } else if set.contains(&ss[i]) {
            return false;
        } else {
            map.insert(pp[i], ss[i]);
            set.insert(ss[i]);
        }
    }


    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_pattern() {
        assert!(word_pattern("abba".to_string(), "dog cat cat dog".to_string()));
        assert!(!word_pattern("abba".to_string(), "dog cat cat fish".to_string()));
        assert!(!word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()));

    }
}