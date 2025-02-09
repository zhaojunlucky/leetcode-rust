use std::collections::HashSet;

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut cols = HashSet::new();
    let mut rows = HashSet::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                cols.insert(j);
                rows.insert(i);
            }
        }
    }

    for c in cols.iter() {
        for i in 0..matrix.len() {
            matrix[i][*c] = 0;

        }
    }

    for r in rows.iter() {
        for j in 0..matrix[0].len() {
            matrix[*r][j] = 0;
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec![vec![0, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0,0]]);

        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
    }
}