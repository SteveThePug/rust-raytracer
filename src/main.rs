#![allow(dead_code)]
#![allow(unused_imports)]
//Use our ray module
mod ray;
use ray::Ray;
//Use our window
mod window;
use window::run;
//Use linear algebra module
use nalgebra::Vector4;
//Use modules for wgpu

//MAIN ---------------------------------------------------------------------------------
fn main() {
    pollster::block_on(run());
}
