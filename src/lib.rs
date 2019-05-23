extern crate pyo3;
extern crate num_cpus;

use std::collections::HashSet;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::sync::Arc;


mod kmeans;

#[pyfunction]
fn kmeans(points: Vec<Vec<f64>>, k: usize, init_method: String, method: String) -> PyResult<Vec<Vec<f64>>> {
    // Regular Kmeans implementation
    fn kmeans_reg(centroids: Vec<Vec<f64>>, points: Vec<Vec<f64>>, iterations: usize, inertia: f64) -> Vec<Vec<f64>>
    {
        let (new_centroids, new_inertia) = kmeans::regular::get_new_centroids(centroids, &points);
        if is_done(&inertia, &new_inertia, &iterations) != false {
            println!("Total iterations: {}", iterations+1);
            return new_centroids
        }
        kmeans_reg(new_centroids, points, iterations+1, new_inertia)
    }

    // Multithreaded Kmeans implementation
    fn kmeans_mult(centroids: Vec<Vec<f64>>, points: &Arc<Vec<Vec<f64>>>, iterations: usize, inertia: f64) -> Vec<Vec<f64>>
    {
        let _num = num_cpus::get();
        let (new_centroids, new_inertia) = kmeans::multithreaded::get_new_centroids(centroids, points, _num);
        if is_done(&inertia, &new_inertia, &iterations) != false {
            println!("Total iterations: {}", iterations+1);
            return new_centroids
        }
        kmeans_mult(new_centroids, points, iterations+1, new_inertia)
    }

    let centroids = init_centroids(&points, k, init_method);

    if method == "multithreaded" {
        let ps = Arc::new(points);
        Ok(kmeans_mult(centroids, &ps, 0, 1.0))
    } else {
        Ok(kmeans_reg(centroids, points, 0, 1.0))
    }
} 

#[pymodule]
fn libedist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(kmeans))?;

    Ok(())
}

fn init_centroids(points: &Vec<Vec<f64>>, k: usize, method: String) -> Vec<Vec<f64>> {

    fn kmeans_pp(points: &Vec<Vec<f64>>, k: usize) -> Vec<Vec<f64>> {
        fn find_next_centroid(dists: &Vec<f64>, rng: &mut ThreadRng) -> usize {
            let mut best_choice: (usize, f64) = (0, 0.0);
            for (i, &j) in dists.iter().enumerate() {
                let value = j * rng.gen::<f64>();
                if value > best_choice.1 {
                    best_choice = (i, value);
                }
            }
            best_choice.0
        }
        
        let mut rng = rand::thread_rng();
        let mut centroids: Vec<Vec<f64>> = Vec::with_capacity(k);
        let mut min_dist_sq: Vec<f64> = vec![std::f64::MAX; points.len()];

        // Establishes the min_dist_sq vector
        centroids.push(points[rng.gen_range(0, points.len()-1)].to_vec());

        for _n in centroids.len().. k {
            for (i, point) in points.iter().enumerate() {
                if let Some(c) = centroids.last() {
                    let dist_sq = c.iter().zip(point).fold(0.0, |acc, (x, p)| acc + (p-x).powi(2));
                    if dist_sq < min_dist_sq[i] {
                        min_dist_sq[i] = dist_sq;
                    }
                } else {
                    panic!("error in if let kmeans_pp")
                }
            }
            centroids.push(points[find_next_centroid(&min_dist_sq, &mut rng)].clone());
        }
        centroids
    }

    fn random(points: &Vec<Vec<f64>>, k: usize) -> Vec<Vec<f64>> {
        let size = Uniform::from(0..points.len()-1);
        let mut rng = rand::thread_rng();
        let mut indices = HashSet::with_capacity(k);
        loop {
            if indices.len() == k {
                break
            }
            indices.insert(size.sample(&mut rng));
        }
        let mut centroids: Vec<Vec<f64>> = Vec::with_capacity(k);
        for i in indices {
            centroids.push(points[i].clone())
        }
        centroids
    }

    if method == "random" {
        println!("Using random init");
        random(points, k)
    } else {
        println!("Using kmeans++");
        kmeans_pp(points, k)
    }
}

fn is_done(inertia: &f64, new_inertia: &f64, c1: &usize) -> bool {
    if *c1 > 300 {
        println!("Final inertia: {}", new_inertia)
        true
    } else if ((new_inertia - inertia).abs()/new_inertia) < 1.0e-5 {
        println!("Final inertia: {}", new_inertia)
        true
    } else {
        false
    }
}
