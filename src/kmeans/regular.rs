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

fn closest_centroids(centroids: &Vec<Vec<f64>>, points: &Vec<Vec<f64>>) -> (Vec<usize>, f64) {
    let mut res: Vec<usize> = Vec::with_capacity(points.len());
    let mut inertia = 0.0;
    for n in points {
        let mut cc: (usize, f64) = (0, std::f64::MAX); // Closest centroid and the squared distance (centroid, dist)

        for (i,j) in centroids.iter().enumerate() {
            let sq_dist: f64 = j.iter().zip(n).fold(0.0, |acc, (x, p)| acc + (p-x).powi(2));
            if sq_dist < cc.1 {
                cc.0 = i;
                cc.1 = sq_dist
            }
        }
        res.push(cc.0);
        inertia += cc.1;
    }
    (res, inertia)
}

fn move_centroids(centroids: Vec<Vec<f64>>, points: &Vec<Vec<f64>>, labels: Vec<usize>) -> Vec<Vec<f64>> {

    let mut mean_loc: Vec<(f64, Vec<f64>)> = vec![(0.0, vec![0.0; centroids[0].len()]); centroids.len()];
    
    for (i,&j) in labels.iter().enumerate() {
        mean_loc[j].1 = points[i].iter().zip(&mean_loc[j].1).map(|(p,x)| p+x).collect();
            
        mean_loc[j].0 = mean_loc[j].0 + 1.0;
    }

    mean_loc.iter().map(|(n, c)| c.into_iter().map(|x| x/n).collect()).collect()
}

pub fn get_new_centroids(centroids: Vec<Vec<f64>>, points: &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, f64) {
    let (labels, inertia) = closest_centroids(&centroids, points);
    (move_centroids(centroids, points, labels), inertia)
}
