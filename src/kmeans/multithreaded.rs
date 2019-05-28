use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

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
