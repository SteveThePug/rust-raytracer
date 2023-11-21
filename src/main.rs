use crate::state::run;
use error_iter::ErrorIter;

const EPSILON: f32 = 1e-6;
const INFINITY: f32 = f32::MAX;
const EPSILON_VECTOR: Vector3<f32> = Vector3::new(EPSILON, EPSILON, EPSILON);

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
