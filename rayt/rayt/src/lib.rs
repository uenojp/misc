pub mod float3;
pub use self::float3::{Color, Float3, Point3, Vec3};

pub mod quat;
pub use self::quat::Quat;

pub mod ray;
pub use self::ray::Ray;

pub mod camera;
pub use self::camera::Camera;

pub mod render;
pub use self::render::{render, render_aa, Scene};

pub use std::f64::consts::{FRAC_1_PI, PI};
pub const PI2: f64 = PI * 2.0;
pub const EPS: f64 = 1e-6;
