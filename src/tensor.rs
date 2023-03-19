// This implementation provides methods for creating tensors with specified shapes and data, 
// creating tensors filled with zeros or ones, creating tensors with random values, reshaping tensors, 
// and performing dot product operations between tensors.
// src/tensor.rs

use ndarray::{Array, ArrayBase, OwnedRepr, Ix1};
//use std::ops::{Add, Sub, Mul, Div};

pub struct Tensor {
    pub data: ArrayBase<OwnedRepr<f32>, Ix1>,
}

impl Tensor {
    // Create a new tensor from a given shape and data
    pub fn new(shape: (usize,), data: Vec<f32>) -> Self {
        Tensor {
            data: Array::from_shape_vec(shape, data).unwrap(),
        }
    }

    // Create a new tensor filled with zeros with a given shape
    pub fn zeros(shape: (usize,)) -> Self {
        Tensor {
            data: Array::zeros(shape),
        }
    }

    // Create a new tensor filled with ones with a given shape
    pub fn ones(shape: (usize,)) -> Self {
        Tensor {
            data: Array::ones(shape),
        }
    }

    // Create a new tensor with random values and a given shape
    pub fn random(shape: (usize,)) -> Self {
        use ndarray_rand::rand_distr::Uniform;
        use ndarray_rand::RandomExt;

        let distribution = Uniform::new(0.0, 1.0);
        let random_array = Array::random(shape, distribution);
        Tensor { data: random_array }
    }

    // Reshape a tensor to a new shape
    pub fn reshape(&self, new_shape: (usize,)) -> Self {
        Tensor {
            data: self.data.clone().into_shape(new_shape).unwrap(),
        }
    }

    // Perform dot product operation between two tensors
    pub fn dot(&self, other: &Self) -> f32 {
        self.data.dot(&other.data)
    }
}

