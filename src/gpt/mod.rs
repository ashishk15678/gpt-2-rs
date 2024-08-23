// Our Transformer stays here
//

use std::collections::HashMap;

use crate::tensor::Tensor;

pub struct TrainingState {
    pub tensors: HashMap<String, Tensor<f32>>,
    //    pub optimizer : Optimizer<>,
}
