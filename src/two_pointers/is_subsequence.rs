fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    for i in 0..t.len() {
        if t.chars().nth(i).unwrap() == s.chars().nth(0).unwrap() {
            let mut j = 1;
            let mut k = i + 1;
            while j < s.len() &&  k < t.len() {
                if t.chars().nth(k).unwrap() == s.chars().nth(j).unwrap() {
                    j += 1;
                }
                k += 1;
            }
            if j == s.len() {
                return true;
            }
        }
    }
    false
}

fn is_subsequence2(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s.chars().nth(i).unwrap() == t.chars().nth(j).unwrap() {
            i += 1;
        }
        j += 1;
    }
    i == s.len()
}

fn is_subsequence3(s: String, t: String) -> bool {
    let ss = s.as_bytes();
    let tt = t.as_bytes();
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if ss[i] == tt[j] {
            i += 1;
        }
        j += 1;
    }
    i == s.len()
}

fn is_subsequence4(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let ss = s.as_bytes();
    let tt = t.as_bytes();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1 ];

    for i in 1..=ss.len() {
        for j in 1..=tt.len() {
            if ss[i - 1] == tt[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
            }
            if dp[i][j] == s.len() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_subsequence("b".to_string(), "dfdbbdd".to_string()), true);

        assert_eq!(is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
        assert_eq!(is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
        assert_eq!(is_subsequence2("b".to_string(), "dfdbbdd".to_string()), true);

        assert_eq!(is_subsequence2("abc".to_string(), "ahbgdc".to_string()), true);
        assert_eq!(is_subsequence2("axc".to_string(), "ahbgdc".to_string()), false);

        assert_eq!(is_subsequence3("b".to_string(), "dfdbbdd".to_string()), true);

        assert_eq!(is_subsequence3("abc".to_string(), "ahbgdc".to_string()), true);
        assert_eq!(is_subsequence3("axc".to_string(), "ahbgdc".to_string()), false);


        assert_eq!(is_subsequence4("abc".to_string(), "ahbgdc".to_string()), true);

        assert_eq!(is_subsequence4("b".to_string(), "dfdbbdd".to_string()), true);

        assert_eq!(is_subsequence4("axc".to_string(), "ahbgdc".to_string()), false);
    }
}