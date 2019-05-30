/* Neural network class. */
use crate::matrix;

#[derive(Debug)]
pub struct NeuralNet {
    in_nodes: i64,                  /* no. of input nodes */
    out_nodes: i64,                 /* no. of output nodes */
    hid_nodes: i64,                 /* no. of hidden nodes */
    lrate: f64,                     /* learning rate */
    wgt_in_to_hid: matrix::Matrix,  /* weights between input and hidden layer */
    wgt_hid_to_out: matrix::Matrix, /* weights between hidden and output layer */
}

impl NeuralNet {
    pub fn new(in_nodes: i64, out_nodes: i64, hid_nodes: i64, lrate: f64)
               -> NeuralNet {
        // initialize matrices with connection weights
        /*
         * TODO: this could be optimized, e.g. draw initial weights from
         *       normal distribution with mean being 0 and standard deviation
         *       being 1/sqrt(input_nodes).
         */
        let mut wgt_in_to_hid: matrix::Matrix = vec![];
        let mut wgt_hid_to_out: matrix::Matrix = vec![];

        matrix::create_rand_matrix(&mut wgt_in_to_hid, hid_nodes, in_nodes);
        matrix::create_rand_matrix(&mut wgt_hid_to_out, out_nodes, hid_nodes);

        // return the initialized neural net
        NeuralNet { in_nodes,
                    out_nodes,
                    hid_nodes,
                    lrate,
                    wgt_in_to_hid,
                    wgt_hid_to_out }
    }

    pub fn _train() {}

    pub fn _query() {}
}
