use nalgebra::{Point3, Vector3};

#[derive(Clone)]
pub struct Light {
    pub position: Point3<f64>,
    pub colour: Vector3<f32>,
    pub falloff: Vector3<f32>,
    pub ambient: bool,
}

impl Light {
    pub fn new(position: Point3<f64>, colour: Vector3<f64>, falloff: Vector3<f64>) -> Light {
        let colour = colour.cast();
        let falloff = falloff.cast();
        Light {
            position,
            colour,
            falloff,
            ambient: false,
        }
    }
    pub fn ambient(colour: Vector3<f64>) -> Light {
        Light {
            position: Point3::new(0.0, 0.0, 0.0),
            colour: colour.cast(),
            falloff: Vector3::new(0.0, 0.0, 0.0),
            ambient: true,
        }
    }
}
