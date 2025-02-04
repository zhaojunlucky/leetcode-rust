fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    while i < j {
        let v = numbers[i] + numbers[j];
        if v == target {
            break
        } else if v < target {
            i += 1
        } else {
            j -= 1
        }
    }
    
    vec![i as i32 + 1, j as i32 + 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![1,2]);
        assert_eq!(two_sum(vec![2,3,4], 6), vec![1,3]);
        assert_eq!(two_sum(vec![-1,0], -1), vec![1,2]);
    }
}