use crate::EPSILON;

use nalgebra::{Matrix4, Point3, Vector3};

pub struct Ray {
    pub a: Point3<f32>,
    pub b: Vector3<f32>,
}

impl Ray {
    pub fn new(_a: Point3<f32>, _b: Vector3<f32>) -> Ray {
        Ray { a: _a, b: _b }
    }
    pub fn at_t(self, t: f32) -> Point3<f32> {
        self.a + self.b * t
    }
}
