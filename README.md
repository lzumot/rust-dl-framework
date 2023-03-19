# Rust Deep Learning Framework

Rust Deep Learning Framework is an open-source deep learning library built with Rust, designed to be fast, efficient, and easy to use. It provides a flexible and intuitive API, allowing users to build, train, and deploy deep learning models with ease.

A deep learning framework written in Rust, inspired by PyTorch. The goal of this project is to provide a fast, safe, and easy-to-use library for creating and training neural networks.

## Features

- High-performance tensor operations using ndarray
- GPU support through [GPU backend library, if applicable]
- Automatic differentiation for gradient computation
- Pre-built layers and activation functions for quick model prototyping
- Custom layer support for advanced use cases
- Model serialization for easy deployment

## Installation

Ensure you have Rust and Cargo installed on your system. If not, follow the instructions at https://www.rust-lang.org/tools/install.

To use Rust Deep Learning Framework in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rust-dl-framework = { git = "https://github.com/lzumot/rust-dl-framework.git" }
```

## Usage
Here's a simple example demonstrating how to create a model, train it, and make predictions:

```rust
use rust_dl_framework::tensor::Tensor;

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0];
    let shape = (4,);
    let tensor = Tensor::new(shape, data);

    println!("{:?}", tensor);
}
```

For more detailed examples and tutorials, check out the examples directory.

## Features
The following features are currently implemented:

Tensors:
Basic tensor operations (addition, subtraction, multiplication, and division)
Creation of tensors from data and shape (1D and 2D)
Initialization methods (zeros, ones, and random)
Reshaping tensors
Utility functions (shape, size, and element access)

## Documentation
The full documentation can be found here.
[Documentation](https://lzumot.github.io/rust-dl-framework/)

## Contributing
We welcome contributions! If you'd like to help improve the Rust Deep Learning Framework, please submit a pull request or open an issue to discuss your ideas.

##  Fork the repository.
Create a new branch with a descriptive name.
Make your changes, ensuring that your code follows the project's style guidelines and passes all tests.
Commit your changes and create a pull request.
For bug reports and feature requests, please open an issue on GitHub.

## License
This project is licensed under the MIT License. See the LICENSE file for details.