// tests/tensor_tests.rs

use rust_dl_framework::tensor::Tensor;

#[test]
fn test_tensor_creation() {
    let data = vec![1.0, 2.0, 3.0, 4.0];
    let tensor = Tensor::new((4,), data.clone());

    assert_eq!(tensor.data.as_slice().unwrap(), data.as_slice());
}

#[test]
fn test_tensor_zeros() {
    let tensor = Tensor::zeros((4,));

    assert_eq!(tensor.data.as_slice().unwrap(), &[0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_tensor_ones() {
    let tensor = Tensor::ones((4,));

    assert_eq!(tensor.data.as_slice().unwrap(), &[1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_tensor_reshape() {
    let data = vec![1.0, 2.0, 3.0, 4.0];
    let tensor = Tensor::new((4,), data);

    let reshaped_tensor = tensor.reshape((2, 2));
    assert_eq!(reshaped_tensor[[0, 0]], 1.0);
    assert_eq!(reshaped_tensor[[0, 1]], 2.0);
    assert_eq!(reshaped_tensor[[1, 0]], 3.0);
    assert_eq!(reshaped_tensor[[1, 1]], 4.0);
}

#[test]
fn test_tensor_dot() {
    let data1 = vec![1.0, 2.0, 3.0, 4.0];
    let tensor1 = Tensor::new((4,), data1);

    let data2 = vec![1.0, 1.0, 1.0, 1.0];
    let tensor2 = Tensor::new((4,), data2);

    let dot_product = tensor1.dot(&tensor2);

    assert_eq!(dot_product, 10.0);
}
