extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn kmeans(centroids: Vec<(i64,i64)>, points: Vec<(i64,i64)>) -> PyResult<Vec<(i64,i64)>> {
    Ok(move_centroids(&centroids, &points))
}

#[pymodule]
fn edist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(kmeans))?;

    Ok(())
}


fn closest_centroids(centroids: &Vec<(i64,i64)>, points: &Vec<(i64,i64)>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(points.len());

    for n in points {
        let mut cc: (usize, i64) = (0, std::i64::MAX); // Closest centroid and the squared distance (centroid, dist)

        for (i,j) in centroids.iter().enumerate() {
            let sq_dists: (Option<i64>, Option<i64>) = ((n.0-j.0).checked_pow(2), (n.1-j.1).checked_pow(2));
            // Matching the squared result to catch integer overflow
            match sq_dists {
                (Some(x), Some(y)) => if x+y < cc.1 {
                    cc.0 = i;
                    cc.1 = x + y
                }
                (None,None) => panic!("Interger overflow occured while squaring {} and {}",n.0-j.0, n.1-j.1),
                (None,Some(_x)) => panic!("Interger overflow occured while squaring {}", n.0-j.0),
                (Some(_x),None) => panic!("Interger overflow occured while squaring {}", n.1-j.1),
            }
        }
        res.push(cc.0);
    }
    res
}

fn move_centroids(centroids: &Vec<(i64,i64)>, points: &Vec<(i64,i64)>) -> Vec<(i64,i64)> {
    let p = closest_centroids(&centroids, &points);
    let mut new_centroids: Vec<(i64,i64)> = Vec::with_capacity(centroids.len());
    let mut mean_loc: Vec<(i64, (i64,i64))> = vec![(0,(0,0)); centroids.len()];
    for (i,&j) in p.iter().enumerate() {
        let (x, y) = points[i];
        mean_loc[j] = (mean_loc[j].0 + 1, ((mean_loc[j].1).0 + x, (mean_loc[j].1).1 + y));
    }
    
    for k in mean_loc {
        new_centroids.push(((k.1).0 / k.0, (k.1).1 / k.0));
    }
    new_centroids
}
