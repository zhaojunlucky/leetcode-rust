fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut i = 0;
    'outer_loop: loop {
        let mut char = ' ';
        for j in 0..strs.len() {
            if i == strs[j].len() {
                break 'outer_loop;
            }
            if char == ' ' {
                char = strs[j].chars().nth(i).unwrap();
            } else if char != strs[j].chars().nth(i).unwrap() {
                break 'outer_loop;
            }
        }
        i += 1;
    }
    strs[0][0..i].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
    }
}