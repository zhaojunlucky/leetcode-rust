fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    let mut tmp = 1;
    for i in 0..nums.len() {
        result[i] = result[i] * tmp;

        tmp *= nums[i];
    }
    tmp = 1;
    for i in (0..nums.len()).rev() {
        result[i] *= tmp;
        tmp *= nums[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::product_except_self;
    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
        assert_eq!(product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    }
}