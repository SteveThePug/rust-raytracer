use crate::camera::Camera;
use crate::light::Light;
use crate::primitive::Primitive;
use nalgebra::Vector3;

pub struct Scene<'a> {
    pub primitives: Vec<Box<dyn Primitive<'a>>>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
    pub ambient_light: Vector3<f32>,
}

impl<'a> Scene<'a> {}
