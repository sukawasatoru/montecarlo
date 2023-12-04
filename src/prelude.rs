pub use anyhow::Result as Fallible;
pub use tracing::{debug, info, Instrument};

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

pub fn calculate_pi(distance_list: &[f64]) -> f64 {
    let valid = distance_list.iter().filter(|data| **data <= 1.0).count();
    (4 * valid) as f64 / distance_list.len() as f64
}
