use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut maps: HashMap<&String, HashMap<&u8, i32>> = HashMap::new();
    let mut ans: Vec<Vec<String>> = vec![];

    for i in 0..strs.len() {
        let mut map = HashMap::new();
        for j in strs[i].as_bytes() {
            *map.entry(j).or_insert(0) += 1;
        }
        maps.insert(&strs[i], map);
    }

    let mut strs2 = strs.clone();
    while !strs2.is_empty() {
        let mut anagrams: Vec<String> = vec![];
        anagrams.push(strs2.remove(0));

        let mut i = 0;
        while i < strs2.len() {
            if maps[&strs2[i]].eq(&maps[&anagrams[0]]) {
                anagrams.push(strs2[i].clone());
                strs2.remove(i);
            } else {
                i += 1;
            }

        }

        ans.push(anagrams);

    }


    ans
}

fn group_anagrams2(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut maps: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for i in 0..strs.len() {
        let mut chars: Vec<char> = strs[i].chars().collect();
        chars.sort();

        maps.entry(chars).or_insert(vec![]).push(strs[i].clone());

    }


    maps.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let strs = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let expected = vec![vec!["eat".to_string(), "tea".to_string(), "ate".to_string()], vec!["tan".to_string(), "nat".to_string()], vec!["bat".to_string()]];
        assert_eq!(group_anagrams(strs), expected);

        assert_eq!(group_anagrams(vec!["".to_string()]), vec![vec!["".to_string()]]);
        assert_eq!(group_anagrams(vec!["a".to_string()]), vec![vec!["a".to_string()]]);
    }
}