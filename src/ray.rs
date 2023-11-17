use crate::{EPSILON, EPSILON_VECTOR};

use nalgebra::{Matrix4, Point3, Unit, Vector3};

pub struct Ray {
    pub a: Point3<f32>,
    pub b: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn new(a: Point3<f32>, b: Unit<Vector3<f32>>) -> Ray {
        Ray { a, b }
    }
    pub fn at_t(&self, t: f32) -> Point3<f32> {
        self.a + self.b.as_ref() * (t + EPSILON)
    }
}
