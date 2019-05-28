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
