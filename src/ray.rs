use crate::EPSILON;

use nalgebra::{Matrix4, Point3, Vector3};

pub struct Ray {
    pub a: Point3<f32>,
    pub b: Vector3<f32>,
}

impl Ray {
    pub fn new(a: Point3<f32>, b: Vector3<f32>) -> Ray {
        let b = b.normalize();
        Ray { a, b }
    }
    pub fn at_t(&self, t: f32) -> Point3<f32> {
        self.a + self.b * t
    }
}
