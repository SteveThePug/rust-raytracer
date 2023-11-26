use nalgebra::{Point3, Vector3};

#[derive(Clone)]
pub struct Light {
    pub position: Point3<f64>,
    pub colour: Vector3<f64>,
    pub falloff: Vector3<f64>,
    pub ambient: bool,
}

impl Light {
    pub fn new(position: Point3<f64>, colour: Vector3<f64>, falloff: Vector3<f64>) -> Self {
        Light {
            position,
            colour,
            falloff,
            ambient: false,
        }
    }
    pub fn ambient(colour: Vector3<f64>) -> Self {
        Light {
            position: Point3::new(0.0, 0.0, 0.0),
            colour,
            falloff: Vector3::new(0.0, 0.0, 0.0),
            ambient: true,
        }
    }
}
