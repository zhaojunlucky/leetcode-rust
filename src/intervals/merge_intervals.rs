fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
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