use nalgebra::{Point3, Vector3};

#[derive(Clone)]
pub struct Light {
    pub position: Point3<f64>,
    pub colour: Vector3<f64>,
    pub falloff: Vector3<f64>,
}

impl Light {
    pub fn new(position: Point3<f64>, colour: Vector3<f64>, falloff: Vector3<f64>) -> Self {
        Light {
            position,
            colour,
            falloff,
        }
    }
    pub fn white(position: Point3<f64>) -> Self {
        let colour = Vector3::new(1.0, 1.0, 1.0);
        let falloff = Vector3::new(1.0, 0.0, 0.0);
        Light::new(position, colour, falloff)
    }
}
