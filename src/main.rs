use crate::state::run;
use error_iter::ErrorIter;

const EPSILON: f64 = 1e-8;
const INFINITY: f64 = 1e10;

use log::error;
use std::env;
use std::error::Error;

mod bvh;
mod camera;
mod gui;
mod light;
mod material;
mod node;
mod primitive;
mod ray;
mod scene;
mod state;

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
