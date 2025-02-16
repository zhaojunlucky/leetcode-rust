fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];

    let mut i = 1;

    let mut left = 0;
    let mut m_right = intervals[left][1];

    while i < intervals.len() {
        if m_right >= intervals[i][0] {
            m_right = intervals[i][1].max(m_right);
            i += 1;
        } else {
            // left, i - 1
            ans.push( vec![intervals[left][0], m_right]);
            left = i;
            m_right = intervals[left][1];
            i+=1;
        }
    }
    ans.push( vec![intervals[left][0], m_right]);

    ans
}

fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    if intervals.len() == 0 {
        return vec![new_interval];
    }
    let mut tmp:Vec<Vec<i32>> = vec![];
    let mut i = 0;

    if new_interval[1] <= intervals[0][0] {
        tmp.push(new_interval.clone());
    } else if new_interval[0] >= intervals[intervals.len()-1][1] {
        tmp = intervals.clone();
        tmp.push(new_interval.clone());
    } else { // has intersection
        while i < intervals.len() {
            if new_interval[0] >= intervals[i][0] && new_interval[1] <= intervals[i][1] {
                // intervals[i][0] <=new_interval[0] new_interval[1] <= intervals[i][1]
                // ignore
                tmp.push(intervals[i].clone());
                break;

            } else if new_interval[0] <= intervals[i][0] && new_interval[1] >= intervals[i][1] {
                // new_interval[0] <= intervals[i][0]  intervals[i][1] <= new_interval[1]
                // replace
                tmp.push(new_interval.clone());
                break;

            } else if i < intervals.len()-1 && new_interval[0] > intervals[i][1] && new_interval[1] < intervals[i+1][0] {
                // new_interval[0] > intervals[i][1] intervals[i+1][0] > new_interval[1]
                // insert
                tmp.push(intervals[i].clone());
                tmp.push(new_interval.clone());
                break;
            } else if intervals[i][1] < new_interval[0] || new_interval[1] < intervals[i][0] {
                // intervals[i][0] intervals[i][1] < new_interval[0] new_interval[1]
                // new_interval[0] new_interval[1] < intervals[i][0] intervals[i][1]
                // no intersection
                tmp.push( intervals[i].clone());

            } else {
                // has intersection
                // intervals[i][0] intervals[i][1]==new_interval[0] new_interval[1]
                //intervals[i][0] new_interval[0] intervals[i][1] new_interval[1]
                // merge
                tmp.push(vec![intervals[i][0].min(new_interval[0]), intervals[i][1].max(new_interval[1])]);
                break;
            }
            i += 1;
        }
        i += 1;

    }
    while i < intervals.len() {
        tmp.push(intervals[i].clone());
        i += 1;
    }

    merge(tmp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let intervals = vec![vec![3, 5], vec![12, 15]];
        let new_interval = vec![6, 6];
        let ans = vec![vec![3, 5], vec![6, 6], vec![12, 15]];
        assert_eq!(insert(intervals, new_interval), ans);


        let intervals = vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]];
        let new_interval = vec![4, 8];
        let ans = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(insert(intervals, new_interval), ans);

        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let ans = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(insert(intervals, new_interval), ans);
    }
}