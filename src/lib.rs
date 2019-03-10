extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn euclidean_dist(centroids: Vec<(i64,i64)>, points: Vec<(i64,i64)>) -> PyResult<Vec<usize>> {
    let mut res = Vec::<usize>::new();

    for n in points {
        let mut cc: (usize, i64) = (0,std::i64::MAX); // Closest centroid and the squared distance (centroid, dist)

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
        res.push(cc.0)
    }
    
    Ok(res)
}

#[pymodule]
fn edist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(euclidean_dist))?;

    Ok(())
}

