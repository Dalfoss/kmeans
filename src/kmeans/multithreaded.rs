/*
Copyright 2019 Morten Torgund Dalfoss.

This file is part of MLfoss.

MLfoss is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

MLfoss is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with MLfoss.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

fn closest_centroids(centroids: &Vec<Vec<f64>>, points: &[Vec<f64>]) -> Vec<(usize, f64, Vec<f64>)> {
    let mut res: Vec<(usize, f64, Vec<f64>)> = Vec::with_capacity(points.len());
    
    for n in points {
        // Closest centroid and the squared distance (centroid, dist)
        match crate::kmeans::euclidean::min_sq_dist(n, centroids) {
            Ok(cc) => res.push((cc.0, cc.1, n.clone())),
            Err(err) => panic!(err),
        }
    }
    res
}

pub fn get_new_centroids(centroids: Vec<Vec<f64>>, points: &Arc<Vec<Vec<f64>>>, cores: usize) -> (Vec<Vec<f64>>, f64) {
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
            inertia += sq_dist;
        }
    }
    (mean_loc.iter().map(|(n, c)| c.into_iter().map(|x| x/n).collect()).collect(), inertia)
}
