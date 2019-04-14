extern crate pyo3;
extern crate num_cpus;

use std::collections::HashSet;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

#[pyfunction]
fn kmeans(points: Vec<Vec<f64>>, k: usize, method: String) -> PyResult<Vec<Vec<f64>>> {
    fn kmeans_inner(centroids: Vec<Vec<f64>>, points: &Arc<Vec<Vec<f64>>>, iterations: usize, inertia: f64) -> Vec<Vec<f64>>
    {
        //        let c_points = closest_centroids(&new_centroids, &points);
        let _num = num_cpus::get();
        let (new_centroids, new_inertia) = get_new_centroids(centroids, points, _num);
        if is_done(&inertia, &new_inertia, &iterations) != false {
            println!("Total iterations: {}", iterations+1);
            return new_centroids
        }
        kmeans_inner(new_centroids, points, iterations+1, new_inertia)
    }
    let centroids = init_centroids(&points, k, method);
    let ps = Arc::new(points);
//    let c_points = closest_centroids(&centroids, &points);
    Ok(kmeans_inner(centroids, &ps, 0, 1.0))
} 

#[pymodule]
fn libedist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(kmeans))?;

    Ok(())
}


fn closest_centroids(centroids: &Vec<Vec<f64>>, points: &[Vec<f64>]) -> Vec<(usize, f64, Vec<f64>)> {
    let mut res: Vec<(usize, f64, Vec<f64>)> = Vec::with_capacity(points.len());
    
    for n in points {
        let mut cc: (usize, f64) = (0, std::f64::MAX); // Closest centroid and the squared distance (centroid, dist)

        for (i,j) in centroids.iter().enumerate() {
            let sq_dist: f64 = j.iter().zip(n).fold(0.0, |acc, (x, p)| acc + (p-x).powi(2));
            if sq_dist < cc.1 {
                cc.0 = i;
                cc.1 = sq_dist
            }
        }
        res.push((cc.0, cc.1, n.clone()));
    }
    res
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

// is_done needs to be reworked if c_points is no longer used
fn is_done(inertia: &f64, new_inertia: &f64, c1: &usize) -> bool {
    if *c1 > 300 {
        true
    } else if ((new_inertia - inertia).abs()/new_inertia) < 1.0e-4 {
        true
    } else {
        println!("{}, {}", (new_inertia - inertia), new_inertia);
        false
    }
}

fn get_new_centroids(centroids: Vec<Vec<f64>>, points: &Arc<Vec<Vec<f64>>>, cores: usize) -> (Vec<Vec<f64>>, f64) {
    let c = Arc::new(centroids);
    let (tx, rx) = mpsc::channel();
    let ppb = points.len()/cores;

    for i in 0..cores {
        let c = Arc::clone(&c);
        let ps = Arc::clone(&points);
        let tx = mpsc::Sender::clone(&tx);
        // Dividing points into n equal sized slices. One for each thread
        if i == cores-1 {
            let _handle = thread::spawn(move || {
                tx.send(closest_centroids(&c, &ps[i*ppb..])).unwrap();
            }); 
        } else {
            let _handle = thread::spawn(move || {
                tx.send(closest_centroids(&c, &ps[i*ppb..(i+1)*ppb])).unwrap();
            });
        }
    }
    drop(tx);
    let mut mean_loc: Vec<(f64, Vec<f64>)> = vec![(0.0, vec![0.0; c[0].len()]); c.len()];
    let mut inertia: f64 = 0.0;
    for recieved in rx {
        for (index, sq_dist, point) in recieved {
            mean_loc[index].0 += 1.0;
            mean_loc[index].1 = point.iter().zip(&mean_loc[index].1).map(|(p,c)| p+c).collect();
            inertia += sq_dist
        }
    }
    (mean_loc.iter().map(|(n, c)| c.into_iter().map(|x| x/n).collect()).collect(), inertia)
}
