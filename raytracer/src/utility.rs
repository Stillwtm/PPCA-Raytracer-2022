pub use crate::vec3::{Vec3, Point3, Color};
pub use crate::ray::Ray;
pub use std::f64::consts::PI;

#[allow(unused)]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}