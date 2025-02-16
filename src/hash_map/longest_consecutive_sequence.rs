use std::collections::HashSet;

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    let mut ans = 0i32;
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for i in 0..nums.len() {
        max = max.max(nums[i]);
        min = min.min(nums[i]);
        set.insert(nums[i]);
    }
    while min <= max {
        let mut cur = min;
        let mut count = 0;
        while set.contains(&cur) {
            count += 1;
            cur += 1;
        }
        ans = ans.max(count);
        if ans > (nums.len()/2) as i32 {
            break;
        }
        min = cur + 1;
    }

    ans
}

fn longest_consecutive2(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    let mut ans = 0i32;

    for i in 0..nums.len() {
        set.insert(nums[i]);
    }
    for no in set.iter() {
        if !set.contains(&(no - 1)) {
            let mut cur = no + 1;
            let mut count = 1;
            while set.contains(&cur) {
                count += 1;
                cur += 1;
            }
            ans = ans.max(count);
        }
    }

    ans
}

fn longest_consecutive3(mut nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0; }

    nums.sort_unstable();
    let mut streak = 1;
    let mut ans = 1i32;

    for i in 1..nums.len() {
        let prev = nums[i-1];
        let curr = nums[i];

        if prev == curr { continue; }
        if prev + 1 == curr {
            streak += 1;
            ans = ans.max(streak);
        } else {
            streak = 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(longest_consecutive(vec![0]), 1);
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);


        assert_eq!(longest_consecutive2(vec![-1000000000,1,2,3,9,1000000000]), 3);
        assert_eq!(longest_consecutive2(vec![0]), 1);
        assert_eq!(longest_consecutive2(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive2(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}