use ndarray::{ArrayD, IxDyn};

pub struct Tensor {
    shape: Vec<usize>,
    data: ArrayD<f32>,
    parents: Vec<Box<Tensor>>,
    requires_grad: bool,
}

impl Tensor {
    pub fn zeros(dims: &[usize]) -> Self {
        let shape: Vec<usize> = dims.to_vec();
        let data = ArrayD::<f32>::zeros(IxDyn(dims));
        Tensor {
            shape,
            data,
            parents: Vec::new(),
            requires_grad: false,
        }
    }
}