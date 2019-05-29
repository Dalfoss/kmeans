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

pub fn is_done(inertia: &f64, new_inertia: &f64, c1: &usize) -> bool {
    if *c1 > 300 {
        println!("Final inertia: {}", new_inertia);
        true
    } else if ((new_inertia - inertia).abs()/new_inertia) < 1.0e-6 {
        println!("Final inertia: {}", new_inertia);
        true
    } else if *c1 == 1 {
        println!("Start inertia: {}", new_inertia);
        false
    } else {
        false
    }
}
