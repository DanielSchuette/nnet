/* Matrix manipulation utilities. */
use rand::Rng;

pub type Matrix = Vec<Vec<f64>>;

pub fn create_rand_matrix(matrix: &mut Matrix, rows: i64, cols: i64) {
    let mut rng = rand::thread_rng();
    for _ in 0..rows {
        let mut new_row: Vec<f64> = vec![];
        for _ in 0..cols {
            let r = rng.gen_range(0.0, 2.0) - 1.0; /* (-1, 1) */
            new_row.push(r);
        }
        matrix.push(new_row);
    }
}

pub fn dot_product(m1: Matrix, m2: Matrix) -> Result<Matrix, String> {
    // check for correct dimensions first
    if m1.len() != m2[0].len() || m1[0].len() != m2.len() {
        return Err(String::from("Failed to multiply matrices with these dimensions"));
    }

    // calculate dot product
    let mut res: Matrix = vec![];
    for row in 0..m1.len() {
        let mut res_row: Vec<f64> = vec![];
        for col in 0..m2[0].len() {
            let mut sum: f64 = 0.0;
            for elem in 0..m1[0].len() {
                sum += m1[row][elem] * m2[elem][col];
            }
            res_row.push(sum);
        }
        res.push(res_row);
    }
    Ok(res)
}

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
    use crate::utils::matrix::{dot_product, transpose};

    #[test]
    fn test_dot_product() {
        // test correct matrix multiplication
        let m = dot_product(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
                            vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);
        let expected = vec![vec![58.0, 64.0], vec![139.0, 154.0]];
        assert!(m.is_ok());
        assert_eq!(m.unwrap(), expected);

        // test input with wrong dimensions
        let m = dot_product(vec![vec![1.0, 2.0]],
                            vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        assert!(m.is_err());
    }

    #[test]
    fn test_transpose() {
        // test correct transposition of matrix
        let m = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let t = transpose(&m);
        let expected = vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]];
        assert_eq!(&t, &expected);

        let tt = transpose(&t);
        assert_eq!(tt, m);
    }
}
