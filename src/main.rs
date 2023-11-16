#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module
use eventloop::run;

//Cameras
mod camera;
mod eventloop;
mod light;
mod primitive;
mod ray;
mod raytracer;
mod scene;
mod state;
// mod state;
// mod texture;
// mod vertex;

const EPSILON: f32 = 1e-7;
const INFINITY: f32 = 1e7;

fn main() {
    pollster::block_on(run());
}
