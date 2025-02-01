fn dp(gas: &Vec<i32>, cost: &Vec<i32>, i: usize, target: usize, oil: i32, cnt: i32) -> i32 {
    let mut j = cnt;
    if i == target {
        if j == 1 {
            return target as i32
        } else {
            j += 1;
        }

    }
    let left = oil - cost[i];
    if left < 0 {
        return -1
    }

    dp(gas, cost, (i + 1) % gas.len(), target, left+gas[(i + 1) % gas.len()], j)
}
fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    for i in 0..gas.len() {
        if dp(&gas, &cost, i, i,gas[i], 0) >= 0 {
            return i as i32
        }
    }
    -1
}

fn can_complete_circuit2(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    for i in 0..gas.len() {
        if gas[i] <= 0 {
            continue
        }
        let mut oil = gas[i];
        let mut pre = i;
        loop {
            oil = oil - cost[pre];
            if oil < 0 {
                // can't reach next
                break;
            }
            pre = (pre + 1) % gas.len();

            if pre == i {
                return i as i32;
            }
            oil = oil + gas[pre];
        }
    }
    -1
}

fn can_complete_circuit3(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_gain = 0;
    let mut cur_gain = 0;
    let mut answer = 0;
    for i in 0..gas.len() {
        total_gain += gas[i] - cost[i];
        cur_gain += gas[i] - cost[i];
        if cur_gain < 0 {
            cur_gain = 0;
            answer = i + 1;
        }
    }

    if total_gain >= 0 {
        answer as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(0, can_complete_circuit3(vec![1], vec![1]));
        assert_eq!(-1, can_complete_circuit3(vec![1], vec![5]));
        assert_eq!(3, can_complete_circuit3(vec![1,2,3,4,5], vec![3,4,5,1,2]));
        assert_eq!(-1, can_complete_circuit3(vec![2,3,4], vec![3,4,3]));
    }
}