/* Neural network class. */
use crate::matrix::*;

const EULERS_NUMBER: f64 = 2.7182818284590452353602874713527;

#[derive(Debug)]
pub struct NeuralNet {
    in_nodes: usize,        /* no. of input nodes */
    out_nodes: usize,       /* no. of output nodes */
    hid_nodes: usize,       /* no. of hidden nodes */
    lrate: f64,             /* learning rate */
    wgt_in_to_hid: Matrix,  /* weights between input and hidden layer */
    wgt_hid_to_out: Matrix, /* weights between hidden and output layer */
    act_fn: fn(f64) -> f64, /* activation function */
}

/// Training and querying the neural net is done via this struct.
impl NeuralNet {
    /// Initialize a new neural net with a given number of input, hidden and
    /// output nodes. The learning rate must be specified, too.
    ///
    /// TODO: this could be optimized, e.g. draw initial weights from normal
    /// distribution with mean being 0 and standard deviation being
    /// `1/sqrt(input_nodes)`.
    pub fn new(in_nodes: usize, out_nodes: usize, hid_nodes: usize, lrate: f64,
               act_fn: fn(f64) -> f64)
               -> NeuralNet {
        let mut wgt_in_to_hid: Matrix = vec![];
        let mut wgt_hid_to_out: Matrix = vec![];

        create_rand_matrix(&mut wgt_in_to_hid, hid_nodes, in_nodes);
        create_rand_matrix(&mut wgt_hid_to_out, out_nodes, hid_nodes);

        // return the initialized neural net
        NeuralNet { in_nodes,
                    out_nodes,
                    hid_nodes,
                    lrate,
                    wgt_in_to_hid,
                    wgt_hid_to_out,
                    act_fn }
    }

    /// Train the neural net. While `data` is a matrix in which every column
    /// represents one training example, `target`'s columns are the respective
    /// solutions which are used for training. An `iter` number of training
    /// iterations is done.
    pub fn train(&mut self, data: Matrix, _target: Matrix, iter: usize) {
        for _ in 0..iter {
            for row in 0..data.len() {
                let d = data[row].clone();
                let _calculated = self.query(&transpose(&vec![d]));
                // TODO: implement backpropagation algorithm
            }
        }
    }

    /// Query the neural net with an input vector (i.e. a matrix with one
    /// column and as many rows as the input layer has neurons). The result
    /// vector is returned to the caller.
    pub fn query(&self, input: &Matrix) -> Result<Matrix, String> {
        // apply weights and activate values twice (in -> hidden -> out)
        let hid_in = dot_product(&self.wgt_in_to_hid, &input)?;
        let hid_out = self.apply_act_fn(&hid_in);
        let final_in = dot_product(&self.wgt_hid_to_out, &hid_out)?;
        let final_out = self.apply_act_fn(&final_in);

        Ok(final_out)
    }

    fn apply_act_fn(&self, m: &Matrix) -> Matrix {
        let mut res: Matrix = vec![];
        for row in 0..m.len() {
            let mut new_row: Vector = vec![];
            for col in 0..m[0].len() {
                let elem = (self.act_fn)(m[row][col]);
                new_row.push(elem);
            }
            res.push(new_row);
        }
        res
    }
}

/// Returns the function value of the sigmoid function for `x`. The curve's
/// maximum value is 1, the midpoint is 0.5.
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + EULERS_NUMBER.powf(-x))
}

#[cfg(test)]
mod tests {
    use crate::nnet::sigmoid;
    use std::f64;

    #[test]
    fn test_sigmoid() {
        let midpoint = sigmoid(0.0);
        let min = sigmoid(-100.0);
        let max = sigmoid(100.0);
        assert!(midpoint - 0.5 <= f64::EPSILON);
        assert!(min - 0.0 <= f64::EPSILON);
        assert!(max - 1.0 <= f64::EPSILON);
    }
}
