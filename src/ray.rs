use nalgebra::Vector4;

#[derive(Copy, Clone)]
pub struct Ray {
    a: Vector4<f32>,
    b: Vector4<f32>,
}
impl Ray {
    fn new(_a: Vector4<f32>, _b: Vector4<f32>) -> Ray {
        Ray { a: _a, b: _b }
    }
    fn point_at_paremeter(&self, t: f32) -> Vector4<f32> {
        self.a + t * self.b
    }
}
