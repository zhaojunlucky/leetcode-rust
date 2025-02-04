use std::collections::HashMap;

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut cache: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut dup = std::collections::HashSet::new();
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..numbers.len() {
        cache.entry(numbers[i]).or_insert(vec![]).push(i);
    }

    for i in 0..numbers.len() -1 {
        let v = target - numbers[i];
        if !cache.contains_key(&v) {
            continue
        }
        let indices = cache.get(&v).unwrap();
        for j in 0..indices.len() {
            if i == indices[j] {
                continue
            }
            let mut arr = vec![numbers[i], numbers[indices[j]]];
            arr.sort();
            if dup.contains(&arr) {
                continue
            }
            dup.insert(arr.clone());
            ans.push(arr);
        }
    }

    ans
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut dup = std::collections::HashSet::new();
    let mut ans: Vec<Vec<i32>> = vec![];

    for i in 0..nums.len() - 2 {
        let two = two_sum(nums[i+1..].to_vec(), -nums[i]);
        for mut v in two {
            v.push(nums[i]);
            v.sort();
            if !dup.contains(&v) {
                dup.insert(v.clone());
                ans.push(v);
            }
        }

    }
    ans
}

fn two_sum2(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    let mut ans: Vec<Vec<i32>> = vec![];

    while i < j {
        let v = numbers[i] + numbers[j];
        if v == target {
            ans.push(vec![numbers[i], numbers[j]]);
            while i < j && numbers[i] == numbers[i+1] {
                i += 1;
            }
            i += 1;
            while i < j && numbers[j] == numbers[j-1] {
                j -= 1;
            }
            if j > 0 {
                j -= 1;
            }
        } else if v < target {
            while i < j && numbers[i] == numbers[i+1] {
                i += 1;
            }
            i += 1;
        } else if v > target {
            while i < j && numbers[j] == numbers[j-1] {
                j -= 1;
            }
            if j > 0 {
                j -= 1;
            }
        }
    }

    ans
}

fn three_sum2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut dup = std::collections::HashSet::new();
    let mut ans: Vec<Vec<i32>> = vec![];

    for i in 0..nums.len() - 2 {
        let two = two_sum2(nums[i+1..].to_vec(), -nums[i]);
        for mut v in two {
            v.push(nums[i]);
            v.sort();
            if !dup.contains(&v) {
                dup.insert(v.clone());
                ans.push(v);
            }
        }


    }
    ans
}

fn two_sum3(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    let mut ans: Vec<Vec<i32>> = vec![];

    while i < j {
        let left = numbers[i];
        let right = numbers[j];
        let v = left + right;
        if v == target {
            ans.push(vec![numbers[i], numbers[j]]);
            while i < j && numbers[i] == left {
                i += 1;
            }
            while i < j && numbers[j] == right {
                j -= 1;
            }
        } else if v < target {
            while i < j && numbers[i] == left {
                i += 1;
            }
        } else if v > target {
            while i < j && numbers[j] == right {
                j -= 1;
            }
        }
    }

    ans
}

fn three_sum3(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut ans: Vec<Vec<i32>> = vec![];
    let mut i = 0;

   while i < nums.len() - 2 && nums[i] <= 0 {
        let target = nums[i];
        let two = two_sum3(nums[i+1..].to_vec(), -target);
        for mut v in two {
            v.push(target);
            ans.push(v);
        }

       while i < nums.len() && nums[i] == target  {
           i += 1;
       }

    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_vec(mut a: Vec<Vec<i32>>, mut b: Vec<Vec<i32>>) {
        for i in 0..a.len() {
            a[i].sort();
        }
        for i in 0..b.len() {
            b[i].sort();
        }
        a.sort();
        b.sort();
        assert_eq!(a, b)
    }

    #[test]
    fn test_three_sum() {

        let empty:Vec<Vec<i32>> = vec![];
        compare_vec(three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
        compare_vec(three_sum(vec![0,1,1]), empty);
        compare_vec(three_sum(vec![0,0,0]), vec![vec![0,0,0]]);

        let empty:Vec<Vec<i32>> = vec![];

        compare_vec(three_sum2(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
        compare_vec(three_sum2(vec![0,1,1]), empty);
        compare_vec(three_sum2(vec![0,0,0]), vec![vec![0,0,0]]);

        let empty:Vec<Vec<i32>> = vec![];

        compare_vec(three_sum3(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
        compare_vec(three_sum3(vec![0,1,1]), empty);
        compare_vec(three_sum3(vec![0,0,0]), vec![vec![0,0,0]]);
    }
}