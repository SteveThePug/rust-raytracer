use crate::state::run;
use error_iter::ErrorIter;

const EPSILON: f64 = 1e-9;
const INFINITY: f64 = f64::MAX;
const EPSILON_VECTOR: Vector3<f64> = Vector3::new(EPSILON, EPSILON, EPSILON);
static ZERO_VECTOR: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
static UP_VECTOR: Vector3<f64> = Vector3::new(0.0, 1.0, 0.0);
static ZERO_VECTOR_F32: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
static UP_VECTOR_F32: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

use log::error;
use std::env;
use std::error::Error;

mod camera;
mod gui;
mod light;
mod primitive;
mod ray;
mod raytracer;
mod scene;
mod state;

use nalgebra::Vector3;

fn main() {
    env_logger::init();

    env::set_var("RUST_BACKTRACE", "1");
    if let Err(e) = run() {
        println!("Error at runtime: {}", e);
    };
}

fn log_error<E: Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
