use std::collections::HashMap;

fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false
    }
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    let ss = s.as_bytes();
    let tt = t.as_bytes();

    for i in 0..ss.len() {
        if map1.contains_key(&ss[i]) {
            if map1[&ss[i]] != tt[i] {
                return false
            }
        } else if map2.contains_key(&tt[i]) {
            return false
        } else {
            map1.insert(ss[i], tt[i]);
            map2.insert(tt[i], ss[i]);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
        assert!(!is_isomorphic("foo".to_string(), "bar".to_string()));
        assert!(is_isomorphic("paper".to_string(), "title".to_string()));
    }
}