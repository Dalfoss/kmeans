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

// Returns the tuple (index, sq_dist)
// index is the index of the point from b that is closest to point.
// sq_dist is the squared euclidean distance from between the two points

pub fn min_sq_dist(point: &Vec<f64>, b: &Vec<Vec<f64>>) -> Result<(usize, f64), &'static str>
{
    if point.len() != b[0].len() {
        return Err("Dimensions doesn't match")
    }

    let mut closest: (usize, f64) = (0, std::f64::MAX);
    for (i,j) in b.iter().enumerate() {
        let sq_dist: f64 = j.iter().zip(point).fold(0.0, |acc, (x, p)| acc + (p-x).powi(2));
        if sq_dist < closest.1 {
            closest.0 = i;
            closest.1 = sq_dist;
        }
    }
    Ok(closest)
}
