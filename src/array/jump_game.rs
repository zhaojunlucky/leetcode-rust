
fn _can_jump2(nums: &Vec<i32>, i: usize, memo: &mut Vec<i32>) -> bool {
    if memo[i] > 0 {
        return memo[i] == 1;
    }

    let max_reach = (i + nums[i] as usize).min(nums.len() - 1);
    for j in i + 1..=max_reach {
        if _can_jump2(nums, j, memo) {
            memo[i] = 1;
            return true
        }
    }
    memo[i] = 2;
    false

}

fn can_jump2(nums: Vec<i32>) -> bool {
    let mut memo = vec![0; nums.len()];
    memo[nums.len()-1] = 1;
    return _can_jump2(&nums, 0, &mut memo);
}

fn can_jump3(nums: Vec<i32>) -> bool {
    let mut memo = vec![0; nums.len()];
    memo[nums.len()-1] = 1;
    for i in (0..=nums.len() - 2).rev() {
        let max_reach = (i + nums[i] as usize).min(nums.len() - 1);
        for j in i + 1..=max_reach {
            if memo[j] == 1 {
                memo[i] = 1;
                break;
            }
        }
    }
    return memo[0] == 1
}

fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_reach = 0;
    for i in 0..nums.len() - 1 {
        max_reach = max_reach.max(i + nums[i] as usize);
        if max_reach <= i {
            return false
        }
    }
    max_reach >= nums.len() - 1
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use super::can_jump;
    use super::can_jump2;
    use super::can_jump3;
    #[test]
    fn test_can_jump() {
        println!("jump 1 {}", Utc::now());
        assert_eq!(can_jump(vec![2,3,1,1,4]), true);
        assert_eq!(can_jump(vec![3,2,1,0,4]), false);
        assert_eq!(can_jump(vec![3,3,1,0,4]), true);
        println!("jump 2 {}", Utc::now());

        assert_eq!(can_jump2(vec![2,3,1,1,4]), true);
        assert_eq!(can_jump2(vec![3,2,1,0,4]), false);
        assert_eq!(can_jump2(vec![3,3,1,0,4]), true);

        println!("jump 3 {}", Utc::now());

        assert_eq!(can_jump3(vec![2,3,1,1,4]), true);
        assert_eq!(can_jump3(vec![3,2,1,0,4]), false);
        assert_eq!(can_jump3(vec![3,3,1,0,4]), true);

        println!("end {}", Utc::now());
    }
}