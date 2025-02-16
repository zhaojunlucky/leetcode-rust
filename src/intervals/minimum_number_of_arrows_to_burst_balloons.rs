fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut ans = 1;

    let mut i = 1;
    let mut cur_right = points[0][1];
    while i < points.len() {
        if cur_right >= points[i][0] {
            i += 1;
        } else {
            ans += 1;
            cur_right = points[i][1];
            i += 1;
        }
    }

    ans

}
