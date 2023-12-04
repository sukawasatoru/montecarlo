use crate::prelude::*;
use rand::prelude::*;

struct PointGenerator {
    rnd: ThreadRng,
}

impl PointGenerator {
    fn new(rnd: ThreadRng) -> PointGenerator {
        PointGenerator { rnd }
    }

    fn generate(&mut self) -> Point {
        Point::new(self.rnd.gen_range(0.0..=1.0), self.rnd.gen_range(0.0..=1.0))
    }
}

pub fn serial(num: usize) -> Fallible<()> {
    let mut generator = PointGenerator::new(thread_rng());

    let mut distance_cache = Vec::with_capacity(num);
    for i in 0..num {
        let point = generator.generate();
        let distance = distance(&point);
        distance_cache.push(distance);
        debug!("{} {}, distance: {:.3}", i, point.flatten_short(), distance);
    }

    println!("{:.5}", calculate_pi(&distance_cache));
    Ok(())
}

fn distance(point: &Point) -> f64 {
    (point.x.powi(2) + point.y.powi(2)).sqrt()
}
