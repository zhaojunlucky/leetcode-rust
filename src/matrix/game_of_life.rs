use std::collections::HashMap;

fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let rows = board.len() - 1;
    let cols = board[0].len() -1;
    let mut dead = HashMap::new();
    let mut live = HashMap::new();

    for i in 0..=rows {
        for j in 0..=cols {
            let mut left = if j > 0 {
                j - 1
            } else {
                0
            };
            let mut right = cols.min(j+1);

            let mut top = if i > 0 {
                i - 1
            } else {
                0
            };

            let mut bottom = rows.min(i+1);

            let mut lives = 0;
            for r in top..=bottom {
                for c in left..=right {
                    if (i != r || j != c) && board[r][c] == 1 {
                        lives += 1;
                    }
                }
            }
            if board[i][j] == 1 && (lives < 2 || lives > 3) {
                dead.insert(i, j);
            } else if board[i][j] == 0 && lives==3 {
                live.insert(i, j);
            }


        }
    }

    for (i, j) in dead {
        board[i][j] = 0;
    }
    for (i, j) in live {
        board[i][j] = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_of_life() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        game_of_life(&mut board);
        assert_eq!(board, vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]);

        let mut board = vec![vec![1, 1], vec![1, 0]];
        game_of_life(&mut board);
        assert_eq!(board, vec![vec![1, 1], vec![1, 1]]);
    }
}