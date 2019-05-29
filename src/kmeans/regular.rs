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

fn closest_centroids(centroids: &Vec<Vec<f64>>, points: &Vec<Vec<f64>>) -> Vec<(usize, f64)> {
    let mut res: Vec<(usize,f64)> = Vec::with_capacity(points.len());
    for n in points {
        // Closest centroid and the squared distance (centroid, dist)
        match crate::kmeans::euclidean::min_sq_dist(n, centroids) {
            Ok(cc) => res.push(cc),
            Err(err) => panic!(err),
        }
    }
    res
}

fn move_centroids(centroids: Vec<Vec<f64>>, points: &Vec<Vec<f64>>, labels: Vec<(usize,f64)>) -> (Vec<Vec<f64>>, f64) {
    let mut inertia = 0.0;
    let mut mean_loc: Vec<(f64, Vec<f64>)> = vec![(0.0, vec![0.0; centroids[0].len()]); centroids.len()];
    
    for (i, &(j, sq_dist)) in labels.iter().enumerate() {
        mean_loc[j].1 = points[i].iter().zip(&mean_loc[j].1).map(|(p,x)| p+x).collect();
            
        mean_loc[j].0 = mean_loc[j].0 + 1.0;
        inertia += sq_dist;
    }

    (mean_loc.iter().map(|(n, c)| c.into_iter().map(|x| x/n).collect()).collect(), inertia)
}

pub fn get_new_centroids(centroids: Vec<Vec<f64>>, points: &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, f64) {
    let labels = closest_centroids(&centroids, points);
    move_centroids(centroids, points, labels)
}
