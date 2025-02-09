use std::collections::HashMap;

fn is_sub_str(mut word_map: HashMap<&String, i32>, string: &String, step: usize) -> bool {
    for i in (0..string.len()).step_by(step) {
        let word = string[i..i + step].to_string();
        if !word_map.contains_key(&word) {
            return false;
        }
        *word_map.get_mut(&word).unwrap() -= 1;
        if *word_map.get(&word).unwrap() == 0 {
            word_map.remove(&word);
        }
    }

    word_map.is_empty()
}
fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let s_l = words.len() * words[0].len();
    if s.len() < s_l {
        return vec![];
    }
    let mut left = 0;
    let mut ans = vec![];
    let mut word_map = HashMap::new();
    let word_len = words[0].len();
    for i in 0..words.len() {
        word_map.entry(&words[i]).or_insert(0);
        *word_map.get_mut(&words[i]).unwrap() += 1;
    }

    for i in s_l-1..s.len() {
        if is_sub_str(word_map.clone(), &s[left..i + 1].to_string(), word_len) {
            ans.push(left as i32);
        }
        left+=1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_substring() {
        assert_eq!(find_substring("barfoothefoobarman".to_string(),
                                  vec!["foo".to_string(), "bar".to_string()]), vec![0, 9]);
        assert_eq!(find_substring("wordgoodgoodgoodbestword".to_string(),
                                  vec!["word".to_string(),"good".to_string(),"best".to_string(),"word".to_string()]), vec![]);
        assert_eq!(find_substring("barfoofoobarthefoobarman".to_string(),
                                  vec!["bar".to_string(),"foo".to_string(),"the".to_string()]), vec![6,9,12]);
    }
}