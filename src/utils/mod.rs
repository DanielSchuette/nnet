/* utils.rs: Utility functions for reading data from a file. */
use crate::matrix::Matrix;

use std::fs;
use std::io::{BufRead, BufReader};

/// Read comma-separated values from a file at `path`. In every row, values `0`
/// to `n-1` are considered data points which are returned as matrix 1. Value
/// `n` is returned as matrix 2, assuming that it represents the target value
/// (see `NeuralNet.train(...)` for more information on how this data is used
/// in the context of the neural net API).
pub fn read_data(path: &str) -> (Matrix, Matrix) {
    let file = fs::File::open(path).expect(&format!("Failed to open file {}", path));
    let file = BufReader::new(&file);
    let mut data: Matrix = vec![];
    let mut target: Matrix = vec![];

    for line in file.lines() {
        let mut new_row_data = vec![];
        let l = line.unwrap();

        for elem in l.split(",") {
            let elem: f64 = elem.parse().unwrap();
            new_row_data.push(elem);
        }

        target.push(vec![new_row_data.pop().unwrap()]);
        data.push(new_row_data);
    }

    (data, target)
}
