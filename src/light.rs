use nalgebra::{Point3, Vector3};

pub struct Light {
    pub colour: Vector3<f32>,
    pub position: Point3<f32>,
    pub falloff: [f32; 3],
}

impl Light {
    pub fn new(colour: Vector3<f32>, position: Point3<f32>, falloff: [f32; 3]) -> Self {
        Light {
            colour,
            position,
            falloff,
        }
    }
    pub fn white() -> Self {
        let colour = Vector3::new(1.0, 1.0, 1.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let falloff = [1.0, 0.0, 0.0];
        Light::new(colour, position, falloff)
    }
}
