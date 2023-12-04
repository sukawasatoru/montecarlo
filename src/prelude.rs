pub use anyhow::Result as Fallible;
use rand::prelude::*;
pub use tracing::{debug, info, trace, Instrument};

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn flatten_short(&self) -> String {
        format!("Point{{x: {:.3}, y: {:.3}}}", self.x, self.y)
    }
}

/// - [Wikipedia](https://ja.wikipedia.org/wiki/モンテカルロ法#数値積分)
/// - [モンテカルロ法で円周率を求めてみよう！](http://www.cmpt.phys.tohoku.ac.jp/open-campus/2020/pi/)
pub fn calculate_pi(distance_list: &[f64]) -> f64 {
    let valid = distance_list.iter().filter(|data| **data <= 1.0).count();
    (4 * valid) as f64 / distance_list.len() as f64
}

pub fn generate_random_point(gen: &mut ThreadRng) -> Point {
    Point::new(gen.gen_range(0.0..=1.0), gen.gen_range(0.0..=1.0))
}

pub fn distance(point: &Point) -> f64 {
    (point.x.powi(2) + point.y.powi(2)).sqrt()
}
