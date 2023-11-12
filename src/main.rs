#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module
use state::run;

//Cameras
mod camera;
mod primitive;
mod ray;
mod state;
mod texture;
mod vertex;

const EPSILON: f32 = 1e-7;

fn main() {
    pollster::block_on(run());
}
