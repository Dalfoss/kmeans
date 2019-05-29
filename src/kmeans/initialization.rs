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

use std::collections::HashSet;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform, Standard};

pub fn init_centroids(points: &Vec<Vec<f64>>, k: usize, method: &String) -> Vec<Vec<f64>> {

    fn kmeans_pp(points: &Vec<Vec<f64>>, k: usize) -> Vec<Vec<f64>> {
        fn find_candidates(dists: &Vec<f64>, samples: usize, rng: &mut ThreadRng) -> Vec<usize> {

            let values: Vec<f64> = dists.iter().zip(rng.sample_iter(&Standard))
                .map(|(d, r):(&f64,f64)| d*r).collect();

            let mut n_samples = vec![(0, 0.0); samples];

            for (index, value) in values.iter().enumerate() {
                if value > &n_samples.last().unwrap().1 {
                    let mut value_index = 0;
                    for (i, entry) in n_samples.iter().enumerate() {
                        if value >= &entry.1 {
                            value_index = i;
                            break
                        }
                    }
                    n_samples.insert(value_index, (index, *value));
                    n_samples.pop();
                }
            }

            // Indicies of the best candidates from points to become a centroid
            n_samples.iter().map(|x| x.0).collect()
        }

        fn best_candidate(points: &Vec<Vec<f64>>, indicies: Vec<usize>) -> usize {
            // Index of the best candidate from points to become a centroid
            let mut best_centroid: usize = 0;
            let mut best_pot: f64 = std::f64::MAX;

            for index in indicies {
                let new_pot = points.iter().map(|p| p.into_iter().zip(&points[index])
                                                .fold(0.0, |acc, (x, c)| acc + (c-x).powi(2))).sum();
                if new_pot < best_pot {
                    best_pot = new_pot;
                    best_centroid = index
                }
            }
            best_centroid
        }

        // Amount of centroid locations to try when choosing each centroid
        let samples = std::mem::size_of::<usize>() * 8 - k.leading_zeros() as usize;
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

            let best_centroid = best_candidate(points,
                                               find_candidates(&min_dist_sq, samples, &mut rng));
            centroids.push(points[best_centroid].clone());
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
