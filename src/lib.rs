extern crate pyo3;

use std::collections::HashSet;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

#[pyfunction]
fn kmeans(points: Vec<(f64,f64)>, k: usize, method: String) -> PyResult<Vec<(f64,f64)>> {
    fn kmeans_inner(centroids: Vec<(f64,f64)>, points: &Vec<(f64,f64)>, last_c_points: Vec<usize>, mut iterations: usize) -> Vec<(f64,f64)> {
        let new_centroids = move_centroids(centroids, points, &last_c_points);
        let c_points = closest_centroids(&new_centroids, &points);
        
        iterations += 1;
        if is_done(&c_points, last_c_points) != false {
            println!("Total iterations: {}", iterations);
            return new_centroids
        }
        kmeans_inner(new_centroids, points, c_points, iterations)
    }
    let centroids = init_centroids(&points, k, method);
    let c_points = closest_centroids(&centroids, &points);
    Ok(kmeans_inner(centroids, &points, c_points, 0))
}


#[pymodule]
fn libedist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(kmeans))?;

    Ok(())
}


fn closest_centroids(centroids: &Vec<(f64,f64)>, points: &Vec<(f64,f64)>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(points.len());

    for n in points {
        let mut cc: (usize, f64) = (0, std::f64::MAX); // Closest centroid and the squared distance (centroid, dist)

        for (i,j) in centroids.iter().enumerate() {
            let sq_dists: (f64, f64) = ((n.0-j.0).powi(2), (n.1-j.1).powi(2));
            // Matching the squared result to catch integer overflow
            if sq_dists.0+sq_dists.1 < cc.1 {
                cc.0 = i;
                cc.1 = sq_dists.0+sq_dists.1
            }
        }
        res.push(cc.0);
    }
    res
}

fn move_centroids(centroids: Vec<(f64,f64)>, points: &Vec<(f64,f64)>, c_points: &Vec<usize>) -> Vec<(f64,f64)> {
    let mut new_centroids: Vec<(f64,f64)> = Vec::with_capacity(centroids.len());
    let mut mean_loc: Vec<(f64, (f64,f64))> = vec![(0.0,(0.0,0.0)); centroids.len()];

    for (i,&j) in c_points.iter().enumerate() {
        let (x, y) = points[i];
        mean_loc[j] = (mean_loc[j].0 + 1.0, ((mean_loc[j].1).0 + x, (mean_loc[j].1).1 + y));
    }
    
    for k in mean_loc {
        new_centroids.push(((k.1).0 / k.0, (k.1).1 / k.0));
    }
    new_centroids
}

fn init_centroids(points: &Vec<(f64,f64)>, k: usize, method: String) -> Vec<(f64,f64)> {

    fn kmeans_pp(points: &Vec<(f64, f64)>, k: usize) -> Vec<(f64,f64)> {
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
        let mut centroids: Vec<(f64,f64)> = Vec::with_capacity(k);
        let mut min_dist_sq: Vec<f64> = Vec::with_capacity(points.len());

        // Establishes the min_dist_sq vector
        let first_centroid = points[rng.gen_range(0, points.len()-1)];
        centroids.push(first_centroid);
        for &point in points {
            match centroids.last() {
                Some((x,y)) => min_dist_sq.push((point.0-x).powi(2) + (point.1-y).powi(2)),
                None => panic!("fejl i kmeans_pp")
            }
        }
        centroids.push(points[find_next_centroid(&min_dist_sq, &mut rng)]);

        for _n in centroids.len().. k {
            for (i, &point) in points.iter().enumerate() {
                if let Some((x,y)) = centroids.last() {
                    let dist_sq = (point.0-x).powi(2) + (point.1-y).powi(2);
                    if dist_sq < min_dist_sq[i] {
                        min_dist_sq[i] = dist_sq;
                    }
                } else {
                    panic!("error in if let kmeans_pp")
                }
            }
            centroids.push(points[find_next_centroid(&min_dist_sq, &mut rng)]);
        }
        centroids
    }

    fn random(points: &Vec<(f64, f64)>, k: usize) -> Vec<(f64,f64)> {
        let size = Uniform::from(0..points.len()-1);
        let mut rng = rand::thread_rng();
        let mut indices = HashSet::with_capacity(k);
        loop {
            if indices.len() == k {
                break
            }
            indices.insert(size.sample(&mut rng));
        }
        let mut centroids: Vec<(f64,f64)> = Vec::with_capacity(k);
        for i in indices {
            centroids.push(points[i])
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

fn is_done(c1: &Vec<usize>, c2: Vec<usize>) -> bool {
    if c2.len() != c1.len() {
        return false 
    }
    let mut moved: f64 = 0.0;
    for (i,&j) in c1.iter().enumerate() {
        if j != c2[i] {
            moved = moved + 1.0;
        }
    }

    if moved/(c1.len() as f64) < 0.0002 {
        true
    } else {
        false
    }
}
