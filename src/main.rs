/* main.rs: Create a neural net, train and query it. */
pub mod matrix;
pub mod nnet;

use matrix::Matrix;
use nnet::{sigmoid, NeuralNet};

fn main() {
    let net: NeuralNet = NeuralNet::new(2, 3, 4, 0.5, sigmoid);
    let pre_train: Matrix = net.query(vec![vec![1.0], vec![2.0]]).unwrap();

    println!("{:#?}", pre_train);
}
