use failure::Fallible;
use log::{debug, info};
use rand::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "montecarlo-pi")]
struct Opt {
    #[structopt(short = "n", long = "num", help = "Plot number")]
    num: usize,
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn flatten_short(&self) -> String {
        format!("Point{{x: {:.3}, y: {:.3}}}", self.x, self.y)
    }
}

struct PointGenerator {
    rnd: ThreadRng,
}

impl PointGenerator {
    fn new(rnd: ThreadRng) -> PointGenerator {
        PointGenerator { rnd }
    }

    fn generate(&mut self) -> Point {
        Point::new(self.rnd.gen_range(0.0, 1.0), self.rnd.gen_range(0.0, 1.0))
    }
}

fn distance(point: &Point) -> f64 {
    (point.x.powi(2) + point.y.powi(2)).sqrt()
}

fn calculate_pi(distance_list: &[f64]) -> f64 {
    let valid = distance_list.iter().filter(|data| **data <= 1.0).count();
    (4 * valid) as f64 / distance_list.len() as f64
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");

    let opt = Opt::from_args() as Opt;

    let mut generator = PointGenerator::new(rand::thread_rng());

    let mut distance_cache = Vec::with_capacity(opt.num);
    for i in 0..opt.num {
        let point = generator.generate();
        let distance = distance(&point);
        distance_cache.push(distance);
        let result = calculate_pi(&distance_cache);
        debug!("{} {}, distance: {:.3}, pi: {:.5}", i, point.flatten_short(), distance, result);
    }

    println!("{:.5}", calculate_pi(&distance_cache));

    info!("Bye");

    Ok(())
}
