/* Matrix manipulation utilities. */
use rand::Rng;

pub type Matrix = Vec<Vec<f64>>;
pub type Vector = Vec<f64>;

/// Appends the specified number of `rows` and `columns` to the matrix `m`. The
/// values are randomly chosen in the exclusive interval `(-1, 1)`.
pub fn create_rand_matrix(m: &mut Matrix, rows: usize, cols: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..rows {
        let mut new_row: Vec<f64> = vec![];
        for _ in 0..cols {
            let r = rng.gen_range(0.0, 2.0) - 1.0; /* (-1, 1) */
            new_row.push(r);
        }
        m.push(new_row);
    }
}

/// Calcuates the dot product of two matrices. Dimension checks are performed
/// such as that a column vector must be passed as `m2` to work with this
/// function.
pub fn dot_product(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String> {
    // check for correct dimensions first
    if !(m1.len() == m2[0].len() && m1[0].len() == m2.len())
       && !(m1[0].len() == m2.len() && m2[0].len() == 1)
    {
        let err_msg = format!("Failed to multiply dimensions {}x{} and {}x{}",
                              m1.len(),
                              m1[0].len(),
                              m2.len(),
                              m2[0].len());
        return Err(err_msg);
    }

    // calculate dot product
    let mut res: Matrix = vec![];
    for row in 0..m1.len() {
        let mut res_row: Vec<f64> = vec![];
        for col_m2 in 0..m2[0].len() {
            let mut sum: f64 = 0.0;
            for col in 0..m1[0].len() {
                sum += m1[row][col] * m2[col][col_m2]
            }
            res_row.push(sum);
        }
        res.push(res_row);
    }
    Ok(res)
}

/// Returns the transpose of input matrix `m`.
pub fn transpose(m: &Matrix) -> Matrix {
    let mut res: Matrix = vec![];

    for col in 0..m[0].len() {
        let mut new_row: Vec<f64> = vec![];
        for row in 0..m.len() {
            new_row.push(m[row][col]);
        }
        res.push(new_row);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::matrix::{dot_product, transpose};

    #[test]
    fn test_dot_product_two_matrices() {
        // test correct matrix multiplication
        let m =
            dot_product(&vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
                        &vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);
        let expected = vec![vec![58.0, 64.0], vec![139.0, 154.0]];
        assert!(m.is_ok());
        assert_eq!(m.unwrap(), expected);

        // test input with wrong dimensions
        let m = dot_product(&vec![vec![1.0, 2.0]],
                            &vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        assert!(m.is_err());
    }

    #[test]
    fn test_dot_product_matrix_vector() {
        let m = dot_product(&vec![vec![1.0, -1.0, 2.0], vec![0.0, -3.0, 1.0]],
                            &vec![vec![2.0], vec![1.0], vec![0.0]]);
        let expected = vec![vec![1.0], vec![-3.0]];
        assert_eq!(m.unwrap(), expected);
    }

    #[test]
    fn test_transpose() {
        // test correct transpose of matrix
        let m = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let t = transpose(&m);
        let expected = vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]];
        assert_eq!(&t, &expected);

        let tt = transpose(&t);
        assert_eq!(tt, m);
    }
}
