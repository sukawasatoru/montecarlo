use crate::prelude::*;
use rand::prelude::*;

pub fn serial(num: usize) -> Fallible<()> {
    let mut gen = thread_rng();

    let mut distance_cache = Vec::with_capacity(num);
    for _ in 0..num {
        let point = generate_random_point(&mut gen);
        let distance = distance(&point);
        trace!(
            point = %point.flatten_short(),
            distance = %format!("{:.3}", distance),
        );
        distance_cache.push(distance);
    }

    println!("{:.5}", calculate_pi(&distance_cache));
    Ok(())
}
