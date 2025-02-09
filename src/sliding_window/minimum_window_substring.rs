use std::collections::HashMap;

fn is_sub(s_map: &HashMap<&u8, i32>, t_map:&HashMap<&u8, i32>) -> bool {
    for (k, v) in t_map {
        if s_map.get(k).unwrap_or(&0) < v {
            return false;
        }
    }
    true
}

fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() {
        return "".to_string();
    }

    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();
    for c in t.as_bytes() {
        let count = t_map.entry(c).or_insert(0);
        *count += 1;
    }
    let mut ans = "";
    let mut left = 0;

    let ss = s.as_bytes();
    for i in 0..ss.len() {
        let count = s_map.entry(&ss[i]).or_insert(0);
        *count += 1;

        while is_sub(&s_map, &t_map) {
            if ans == "" || ans.len() > (i - left + 1) {
                ans = &s[left..=i];
            }
            let count = s_map.entry(&ss[left]).or_insert(0);
            *count -= 1;
            if *count <= 0 {
                s_map.remove(&ss[left]);
            }
            left += 1;
        }

    }

    ans.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a".to_string());
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "".to_string());
    }
}