use std::collections::{HashMap, HashSet};

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    let mut ans = vec![];
    for i in 0..nums.len() {
        map.entry(nums[i]).or_insert(HashSet::new()).insert(i);
    }

    for i in 0..nums.len() {
        if let Some(count) = map.get(&(target - nums[i])) {
            if !count.contains(&i) {
                ans = vec![i as i32, (*count.iter().next().unwrap()) as i32];
                break;

            } else if count.len() > 1 {
                ans = vec![i as i32, *(count.iter().filter(|x| **x != i).next().unwrap()) as i32];
                break
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}