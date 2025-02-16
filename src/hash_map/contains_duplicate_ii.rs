use std::collections::HashMap;

fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::new();

    for i in 0..nums.len() {
        let v = map.entry(nums[i]).or_insert(vec![]);
        v.push(i);
    }

    for (_, indexes) in map {
        if indexes.len() < 2 {
            continue;
        }
        for j in (1..indexes.len()).rev() {
            for i in (0..j).rev() {
                if indexes[j] - indexes[i] <= k as usize {
                    return true;
                } else {
                    break;
                }
            }
        }

    }
    false
}