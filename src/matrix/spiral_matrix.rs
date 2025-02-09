fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }
    let mut ans = vec![];
    let mut left = 0 as i32;
    let mut right = matrix[0].len() as i32 - 1;
    let mut top = 0 as i32;
    let mut bottom = matrix.len() as i32 - 1;
    let mut direction = 1;

    while left <= right && top <= bottom {
        if direction == 1 {
            for i in left..=right {
                ans.push(matrix[top as usize][i as usize]);
            }
            top += 1;
        }

        if direction == 2 {
            for i in top ..=bottom {
                ans.push( matrix[i as usize][right as usize] );
            }
            right -= 1;
        }

        if direction == 3 {
            for i in (left..=right).rev() {
                ans.push( matrix[bottom as usize][i as usize] );
            }
            bottom -= 1;

        }

        if direction == 4 {
            for i in (top..=bottom).rev() {
                ans.push( matrix[i as usize][left as usize] );
            }
            left += 1;
        }
        direction += 1;
        direction %= 4;

    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let matrix = vec![vec![3], vec![2]];
        let expected = vec![3,2];
        assert_eq!(spiral_order(matrix), expected);

        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        // assert_eq!(spiral_order(matrix), expected);

        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(spiral_order(matrix), expected);
    }
}
