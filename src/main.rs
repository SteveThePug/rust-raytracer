#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module
use window::run;

//Cameras
mod camera;
mod light;
mod primitive;
mod ray;
mod raytracer;
mod scene;
mod window;
// mod state;
// mod texture;
// mod vertex;

const EPSILON: f32 = 1e-7;
const INFINITY: f32 = 1e7;

fn main() {
    run().expect("YEY");
}
