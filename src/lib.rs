extern crate pyo3;
extern crate num_cpus;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::Arc;


mod kmeans;

#[pyfunction]
fn kmeans(points: Vec<Vec<f64>>, k: usize, init_method: String, method: String) -> PyResult<Vec<Vec<f64>>> {
    // Regular Kmeans implementation
    fn kmeans_reg(centroids: Vec<Vec<f64>>, points: &Vec<Vec<f64>>, iterations: usize, inertia: f64) -> (Vec<Vec<f64>>, f64)
    {
        let (new_centroids, new_inertia) = kmeans::regular::get_new_centroids(centroids, &points);
        if kmeans::termination::is_done(&inertia, &new_inertia, &iterations) != false {
            println!("Total iterations: {}", iterations+1);
            return (new_centroids, new_inertia)
        }
        kmeans_reg(new_centroids, points, iterations+1, new_inertia)
    }

    // Multithreaded Kmeans implementation
    fn kmeans_mult(centroids: Vec<Vec<f64>>, points: &Arc<Vec<Vec<f64>>>, iterations: usize, inertia: f64) -> (Vec<Vec<f64>>, f64)
    {
        let _num = num_cpus::get();
        let (new_centroids, new_inertia) = kmeans::multithreaded::get_new_centroids(centroids, points, _num);
        if kmeans::termination::is_done(&inertia, &new_inertia, &iterations) != false {
            println!("Total iterations: {}", iterations+1);
            return (new_centroids, new_inertia)
        }
        kmeans_mult(new_centroids, points, iterations+1, new_inertia)
    }

    let mut result = (Vec::<Vec<f64>>::with_capacity(5), std::f64::MAX);
    for _i in 0..10 {
        let centroids = kmeans::initialization::init_centroids(&points, k, &init_method);

        let temp = if method == "multithreaded" {
            let ps = Arc::new(points.clone());
            kmeans_mult(centroids, &ps, 0, 1.0)
        } else {
            kmeans_reg(centroids, &points, 0, 1.0)
        };
        if temp.1 < result.1 {
            result = temp
        }
    }
    Ok(result.0)
}


#[pymodule]
fn libedist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(kmeans))?;

    Ok(())
}
