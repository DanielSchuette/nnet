/* main.rs: Create a neural net, train and query it. */
pub mod matrix;
pub mod nnet;
pub mod utils;

use matrix::Matrix;
use nnet::{sigmoid, NeuralNet};
use utils::read_data;

const PATH: &str = "data/train";

fn main() {
    // get training data from a file
    let data = read_data(PATH);

    // create a neural net, train it and print the output
    // (input and output layer sized must be chosen appropriately, because
    // there are *no* internal checks for consistency)
    let exmpl = vec![vec![1.0], vec![0.0]];
    let mut net: NeuralNet = NeuralNet::new(2, 1, 4, 0.5, sigmoid);
    let _pre_train: Matrix = net.query(&exmpl).unwrap();
    net.train(data.0, data.1, 1);
    let _post_train: Matrix = net.query(&exmpl).unwrap();

    // compare the resulting matrices
    println!("{:?}", _pre_train);
    println!("{:?}", _post_train);
}
