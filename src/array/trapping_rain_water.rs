fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut result = 0;
    for i in 1..n {
        left[i] = left[i-1].max(height[i-1]);
    }
    for i in (0..n-1).rev() {
        right[i] = right[i+1].max(height[i+1]);
    }
    for i in 0..n {
        result += (left[i].min(right[i]) - height[i]).max(0);
    }
    result
}

fn trap1(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut ans = 0;
    let mut left_max = 0;
    let mut right_max = 0;

    while left < right{
        if height[left] < height[right] {
            if height[left] > left_max {
                left_max = height[left];
            } else {
                ans += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] > right_max {
                right_max = height[right];
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }

    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(trap(vec![4,2,0,3,2,5]), 9);
        assert_eq!(trap1(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(trap1(vec![4,2,0,3,2,5]), 9);
    }
}