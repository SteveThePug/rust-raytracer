use crate::camera::Camera;
use crate::light::Light;
use crate::primitive::Primitive;
use nalgebra::Vector3;
use std::sync::Arc;

pub struct Scene {
    pub primitives: Arc<Vec<Box<dyn Primitive>>>,
    pub lights: Arc<Vec<Light>>,
    pub cameras: Arc<Vec<Camera>>,
    pub ambient_light: Arc<Vector3<f32>>,
}

impl Scene {
    // Creates a scene
    pub fn new(
        primitives: Vec<Box<dyn Primitive>>,
        lights: Vec<Light>,
        cameras: Vec<Camera>,
        ambient_light: Vector3<f32>,
    ) -> Self {
        Scene {
            primitives: Arc::new(primitives),
            lights: Arc::new(lights),
            cameras: Arc::new(cameras),
            ambient_light: Arc::new(ambient_light),
        }
    }
}
