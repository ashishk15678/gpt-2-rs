use thiserror::Error;

use crate::tensor::{Tensor, TensorError};

pub trait Graph {
    //    fn alloc(&mut self, t: Tensor<f32>, is_param: bool, name: String) -> Result<usize, GraphError> {
    //  }
}

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("tensor error {0}")]
    TensorError(#[from] TensorError),
}
