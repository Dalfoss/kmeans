# kmeans
The purpose of this project is to implement a version of kmeans as a python
module written in Rust.  The goal is a better performing kmeans module for
python, than is currently available through packages like sklearn.

## Usage
### Build
Due to the use of the pyo3 crate, it requires a nightly version of rustc to build this package.
Luckily the version of your rust compiler is easily handled by Rustup.

To install Rustup follow the instructions at https://www.rust-lang.org/tools/install

The following build command should be used while in the root directory of this repository `cargo build --release` to build the package.

### Using the module
When you have build the package, you can move the dynamic library that is the output of the compilation, to the directory of your python project, and import it with `import libedist`.

an example of modules use, can be found in the benchmark script in `benches/pythonbench.py`.

## Future plans for distribution
When the project is more mature, it is planned to distribute the package via Pypi, to be installed with pip as any other python module.
