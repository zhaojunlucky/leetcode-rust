fn convert(s: String, num_rows: i32) -> String {
    let mut ans: Vec<String> = vec![String::new(); num_rows as usize];

    let mut i:i32 = 0;
    let mut direction:i32 = 1;
    for c in s.chars() {
        ans[i as usize].push(c);
        if i == num_rows -1 {
            direction = -1;
        } else if i == 0 {
            direction = 1;
        }
        i += direction;

        if i == -1 {
            i = 0;
        }
    }

    ans.join("")
}

fn convert2(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        return s;
    }
    let mut ans: Vec<String> = vec![String::new(); num_rows as usize];

    let mut i:i32 = 0;
    let mut direction:i32 = 1;
    for c in s.chars() {
        ans[i as usize].push(c);
        if i == num_rows -1 {
            direction = -1;
        } else if i == 0 {
            direction = 1;
        }
        i += direction;
    }

    ans.join("")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
        assert_eq!(convert("A".to_string(), 1), "A".to_string());
        assert_eq!(convert("AB".to_string(), 1), "AB".to_string());
    }
}