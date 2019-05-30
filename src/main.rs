/* main.rs: Create a neural net, train and evaluate it. */
pub mod matrix;
pub mod nnet;

use nnet::NeuralNet;

fn main() {
    let net: NeuralNet = NeuralNet::new(2, 3, 4, 0.5);
    println!("{:#?}", net);
}
