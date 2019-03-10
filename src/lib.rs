extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn kmeans(centroids: Vec<(f64,f64)>, points: Vec<(f64,f64)>) -> PyResult<Vec<(f64,f64)>> {
    Ok(move_centroids(&centroids, &points))
}

#[pymodule]
fn edist(_py: Python, m: &PyModule) -> PyResult<()> {
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
                    cc.1 = sq_dists.0 + sq_dists.1
            }
        }
        res.push(cc.0);
    }
    res
}

fn move_centroids(centroids: &Vec<(f64,f64)>, points: &Vec<(f64,f64)>) -> Vec<(f64,f64)> {
    let p = closest_centroids(&centroids, &points);
    let mut new_centroids: Vec<(f64,f64)> = Vec::with_capacity(centroids.len());
    let mut mean_loc: Vec<(f64, (f64,f64))> = vec![(0.0,(0.0,0.0)); centroids.len()];

    for (i,&j) in p.iter().enumerate() {
        let (x, y) = points[i];
        mean_loc[j] = (mean_loc[j].0 + 1.0, ((mean_loc[j].1).0 + x, (mean_loc[j].1).1 + y));
    }
    
    for k in mean_loc {
        new_centroids.push(((k.1).0 / k.0, (k.1).1 / k.0));
    }
    new_centroids
}
