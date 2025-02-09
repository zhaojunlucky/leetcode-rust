use std::collections::HashSet;

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut boxes = vec![HashSet::new(), HashSet::new(), HashSet::new()];
    for i in 0..9 {
        let mut row = HashSet::new();
        let mut col = HashSet::new();
        if i % 3 == 0 {
            boxes[0].clear();
            boxes[1].clear();
            boxes[2].clear();
        }
        for j in 0..9 {

            if board[i][j] != '.' {
                if row.contains(&board[i][j]) {
                    return false;
                }
                row.insert(&board[i][j]);

                if boxes[j/3].contains(&board[i][j]) {
                    return false;
                }
                boxes[j/3].insert(&board[i][j]);
            }
            if board[j][i] != '.' {
                if col.contains(&board[j][i]) {
                    return false;
                }
                col.insert(&board[j][i]);
            }


        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        assert!(is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]));
        assert!(!is_valid_sudoku(vec![vec!['8','3','.','.','7','.','.','.','.']
                                      ,vec!['6','.','.','1','9','5','.','.','.']
                                      ,vec!['.','9','8','.','.','.','.','6','.']
                                      ,vec!['8','.','.','.','6','.','.','.','3']
                                      ,vec!['4','.','.','8','.','3','.','.','1']
                                      ,vec!['7','.','.','.','2','.','.','.','6']
                                      ,vec!['.','6','.','.','.','.','2','8','.']
                                      ,vec!['.','.','.','4','1','9','.','.','5']
                                      ,vec!['.','.','.','.','8','.','.','7','9']]));
    }
}