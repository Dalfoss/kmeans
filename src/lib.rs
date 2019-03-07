extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn euclidean_dist(centroids: Vec<(i64,i64)>, points: Vec<(i64,i64)>) -> PyResult<Vec<Vec<i64>>> {
    let mut res = Vec::<Vec<i64>>::new();

    for n in centroids {
        let mut row = Vec::<i64>::new();
        for i in &points {
            let sq_dists: (Option<i64>, Option<i64>) = ((i.0-n.0).checked_pow(2), (i.1-n.1).checked_pow(2));
            
            // Matching the squared result to catch integer overflow
            match sq_dists {
                (Some(x), Some(y)) => row.push(x + y),
                (None,None) => panic!("Interger overflow occured while squaring {} and {}",n.0-i.0, n.1-i.1),
                (None,Some(_x)) => panic!("Interger overflow occured while squaring {}", n.0-i.0),
                (Some(_x),None) => panic!("Interger overflow occured while squaring {}", n.1-i.1),
            }
        }
        res.push(row)
    }
    
    Ok(res)
}

#[pymodule]
fn edist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(euclidean_dist))?;

    Ok(())
}
