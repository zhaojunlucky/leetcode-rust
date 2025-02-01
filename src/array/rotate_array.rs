pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut k = k % nums.len() as i32;

    let flip : fn (nums1: &mut Vec<i32>, start: &mut i32, end: & mut i32) = |nums1, start, end| {
        while start < end {
            nums1.swap(*start as usize, *end as usize);
            *start += 1;
            *end -= 1;
        }
    };
    flip(nums, &mut 0, &mut (nums.len() as i32 - 1));
    flip(nums, &mut 0, &mut (k - 1));
    flip(nums, &mut k, &mut (nums.len() as i32 - 1));

}

pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
    let k = k % nums.len() as i32;

    let flip : fn (nums1: &mut Vec<i32>, start: &mut i32, end: & mut i32) = |nums1, start, end| {
        while start < end {
            nums1.swap(*start as usize, *end as usize);
            *start += 1;
            *end -= 1;
        }
    };
    // flip last k
    flip(nums, &mut (nums.len() as i32 - k), &mut (nums.len() as i32 - 1));
    // flip first nums.len() - k
    flip(nums, &mut 0, &mut (nums.len() as i32 -k - 1));
    flip(nums, &mut 0, &mut (nums.len() as i32 - 1));

}

#[cfg(test)]
mod tests {
    fn _test_rotate(nums: &mut Vec<i32>, k: i32, expect: Vec<i32>) {
        println!("before {:#?}", nums);
        super::rotate2(nums, k);
        println!("after {:#?}", nums);
        assert_eq!(expect, nums.to_vec());
        println!()
    }
    #[test]
    fn test_rotate() {
        _test_rotate(&mut vec![1, 2], 1, vec![2, 1]);
        _test_rotate(&mut vec![1], 1, vec![1]);
        _test_rotate(&mut vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]);
        _test_rotate(&mut vec![-1, -100, 3, 99], 2 , vec![3, 99, -1, -100]);
    }
}