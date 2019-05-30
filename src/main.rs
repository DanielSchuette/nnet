/* main.rs: Create a neural net, train and evaluate it. */
pub mod matrix;
pub mod nnet;

use nnet::{sigmoid, NeuralNet};

fn main() {
    let net: NeuralNet = NeuralNet::new(2, 3, 4, 0.5, sigmoid);
    println!("{:#?}", net.query(vec![vec![1.0], vec![2.0]]));
}
