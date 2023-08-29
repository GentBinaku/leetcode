use crate::medium::spiral_matrix::Direction::Right;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut direction = Direction::Right;
    let mut round = 0;
    let mut row = 0;
    let mut col = 0;
    let m = matrix.len();
    let n = matrix[0].len();
    let mut flatten_matrix = vec![];

    for _ in 0..m * n {
        flatten_matrix.push(matrix[row][col]);

        match direction {
            Direction::Right => {
                if col + 1 == n - round {
                    direction = Direction::Down;
                    row += 1;
                } else {
                    col += 1;
                }
            }
            Direction::Left => {
                if col == round {
                    direction = Direction::Up;
                    row -= 1;
                } else {
                    col -= 1;
                }
            }
            Direction::Down => {
                if row + 1 == m - round {
                    direction = Direction::Left;
                    col -= 1;
                } else {
                    row += 1;
                }
            }
            Direction::Up => {
                if row == round + 1 {
                    direction = Direction::Right;
                    col += 1;
                    round += 1;
                } else {
                    row -= 1;
                }
            }
        }
    }
    flatten_matrix
}



#[cfg(test)]
mod test {
    use crate::medium::spiral_matrix;
    use crate::medium::spiral_matrix::spiral_order;

    #[test]
    fn should_return_true() {
        assert_eq!(spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }
}