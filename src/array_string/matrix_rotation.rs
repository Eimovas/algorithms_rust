
#[allow(dead_code)]
fn rotate<'a>(mut matrix : Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let size = matrix.len();

    for i in 0..size {
        for j in (i+1)..size {
            let row = matrix[i][j];
            let col = matrix[j][i];
            matrix[i][j] = col;
            matrix[j][i] = row;
        }
    }

    return matrix;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_matrix_when_small_then_return_valid() {
        let matrix = vec![vec![1]];
        let result = rotate(matrix);
        assert_eq!(vec![vec![1]], result);
    }

    #[test]
    fn given_matrix_when_large_then_return_valid() {
        let matrix = vec![
            vec![1,2,3,4],
            vec![1,2,3,4],
            vec![1,2,3,4],
            vec![1,2,3,4]
        ];
        let expected = vec![
            vec![1,1,1,1],
            vec![2,2,2,2],
            vec![3,3,3,3],
            vec![4,4,4,4]
        ];

        let result = rotate(matrix);
        assert_eq!(expected, result);
    }
}