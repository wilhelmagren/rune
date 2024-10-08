use ndarray::

pub struct Tensor {
    shape: Vec<usize>,
    data: ArrayD<f32>,
    parents: Vec<Box<Tensor>>,
    requires_grad: bool,
}

impl Tensor {
    pub fn zeros(dims: &[usize]) -> Self {
        Tensor {
        }
    }
}