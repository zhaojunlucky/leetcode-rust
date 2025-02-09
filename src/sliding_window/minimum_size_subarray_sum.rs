
fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut cur_sum = 0;
    let mut ans = usize::MAX;

    for i in 0..nums.len() {
        cur_sum += nums[i];
        while cur_sum >= target {
            ans = ans.min(i - left + 1);
            cur_sum -= nums[left];
            left += 1;
            continue
        }

    }

    if ans == usize::MAX {
        return 0;
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}