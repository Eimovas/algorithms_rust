#[allow(dead_code)]
fn set_zero(mut matrix : Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.len() == 0 {
        return matrix;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut indexes : Vec<(usize,usize)> = Vec::new();

    // find zero addresses
    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 0 {
                indexes.push((i,j));
            }
        }
    }

    // update matrix
    for (x,y) in indexes {
        for i in 0..rows {
            matrix[i][y] = 0;
        }
        for j in 0..cols {
            matrix[x][j] = 0;
        }
    }

    matrix
}

#[allow(dead_code)]
fn set_zero_efficient(mut matrix : Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.len() == 0 {
        return matrix;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut zero_rows: Vec<usize> = Vec::new();
    let mut zero_cols: Vec<usize> = Vec::new();

    // find zero addresses
    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 0 {
                zero_rows.push(i);
                zero_cols.push(j);
            }
        }
    }

    for row in zero_rows {
        for i in 0..cols {
            matrix[row][i] = 0;
        }
    }

    for col in zero_cols {
        for i in 0..rows {
            matrix[i][col] = 0;
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_matrix_when_empty_then_return_empty() {
        let input: Vec<Vec<i32>> = vec![];
        let expected : Vec<Vec<i32>> = vec![];
        assert_eq!(expected, set_zero_efficient(input));
    }
    #[test]
    fn given_matrix_when_n_gt_m_then_return_valid() {
        let input: Vec<Vec<i32>> = vec![
            vec![1,2,3,4],
            vec![1,2,3,0],
            vec![1,2,3,4]
        ];
        let expected : Vec<Vec<i32>> = vec![
            vec![1,2,3,0],
            vec![0,0,0,0],
            vec![1,2,3,0]
        ];
        assert_eq!(expected, set_zero_efficient(input));
    }
    #[test]
    fn given_matrix_when_m_gt_n_then_return_valid() {
        let input: Vec<Vec<i32>> = vec![
            vec![1,2,3],
            vec![1,0,3],
            vec![1,2,3],
            vec![1,2,3]
        ];
        let expected : Vec<Vec<i32>> = vec![
            vec![1,0,3],
            vec![0,0,0],
            vec![1,0,3],
            vec![1,0,3]
        ];
        assert_eq!(expected, set_zero_efficient(input));
    }
    #[test]
    fn given_matrix_when_m_equals_n_then_return_valid() {
        let input: Vec<Vec<i32>> = vec![
            vec![1,2,3,4],
            vec![1,0,3,4],
            vec![1,2,3,4],
            vec![1,2,3,4]
        ];
        let expected : Vec<Vec<i32>> = vec![
            vec![1,0,3,4],
            vec![0,0,0,0],
            vec![1,0,3,4],
            vec![1,0,3,4]
        ];
        assert_eq!(expected, set_zero_efficient(input));
    }
    #[test]
    fn given_matrix_when_multiple_zeroes_then_return_valid() {
        let input: Vec<Vec<i32>> = vec![
            vec![1,2,3,4],
            vec![1,0,3,4],
            vec![1,2,3,4],
            vec![1,2,3,0]
        ];
        let expected : Vec<Vec<i32>> = vec![
            vec![1,0,3,0],
            vec![0,0,0,0],
            vec![1,0,3,0],
            vec![0,0,0,0]
        ];
        assert_eq!(expected, set_zero_efficient(input));
    }

    #[test]
    fn given_matrix_when_multiple_zeroes_in_same_row_then_return_valid() {
        let input: Vec<Vec<i32>> = vec![
            vec![1,2,3,4],
            vec![1,0,3,0],
            vec![1,2,3,4],
            vec![1,2,3,4],
            vec![1,2,3,4]
        ];
        let expected : Vec<Vec<i32>> = vec![
            vec![1,0,3,0],
            vec![0,0,0,0],
            vec![1,0,3,0],
            vec![1,0,3,0],
            vec![1,0,3,0]
        ];
        assert_eq!(expected, set_zero_efficient(input));
    }

    #[test]
    fn given_matrix_when_all_zeroes_then_return_valid() {
        let input: Vec<Vec<i32>> = vec![
            vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0]
        ];
        let expected : Vec<Vec<i32>> = vec![
            vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0]
        ];
        assert_eq!(expected, set_zero_efficient(input));
    }

    #[test]
    fn given_matrix_when_all_non_zeroes_then_return_valid() {
        let input: Vec<Vec<i32>> = vec![
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1]
        ];
        let expected : Vec<Vec<i32>> = vec![
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1]
        ];
        assert_eq!(expected, set_zero_efficient(input));
    }
}
