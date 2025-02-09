use std::collections::HashMap;

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut magazine_map = HashMap::new();
    for b in magazine.as_bytes() {
        let count = magazine_map.entry(b).or_insert(0);
        *count += 1;
    }

    for b in ransom_note.as_bytes() {
        if let Some(count) = magazine_map.get_mut(&b) {
            if *count > 0 {
                *count -= 1;
            } else {
                return false;
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
    fn test_can_construct() {
        assert!(!can_construct("a".to_string(), "b".to_string()));
        assert!(!can_construct("aa".to_string(), "ab".to_string()));
        assert!(can_construct("aa".to_string(), "aab".to_string()));
    }
}