fn roman_to_int(s: String) -> i32 {
    let mut ans = 0;
    let roman = |c: char| -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0
        }
    };

    for i in 0..s.len() {
        let cur = roman(s.chars().nth(i).unwrap());
        if i < s.len() - 1 {
            let next = roman(s.chars().nth(i + 1).unwrap());
            if cur < next {
                ans -= cur;
            } else {
                ans += cur;
            }
        } else {
            ans += cur;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(3, roman_to_int("III".to_string()));
        assert_eq!(58, roman_to_int("LVIII".to_string()));
        assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));
    }
}