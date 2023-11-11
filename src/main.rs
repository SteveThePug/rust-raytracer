#![allow(dead_code)]
mod ray;
use nalgebra::{Matrix4, Vector4};

use ray::Ray;

fn main() {
    let x = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let mat_a = Matrix4::from_diagonal_element(0.12);
    println!("mat_a*x = {}", { mat_a * x });
}
