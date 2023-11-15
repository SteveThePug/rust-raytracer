use crate::camera::Camera;
use crate::light::Light;
use crate::primitive::Primitive;
use nalgebra::Vector3;
use std::sync::Arc;

pub struct Scene {
    pub primitives: Vec<Arc<dyn Primitive>>,
    pub lights: Vec<Arc<Light>>,
    pub cameras: Vec<Arc<Camera>>,
    pub ambient_light: Arc<Vector3<f32>>,
}

impl Scene {
    // Creates a new Scene with given parameters
    pub fn new(
        primitives: Vec<Arc<dyn Primitive>>,
        lights: Vec<Arc<Light>>,
        cameras: Vec<Arc<Camera>>,
        ambient_light: Arc<Vector3<f32>>,
    ) -> Self {
        Scene {
            primitives,
            lights,
            cameras,
            ambient_light,
        }
    }

    // Creates an empty Scene with default values
    pub fn empty() -> Self {
        Scene {
            primitives: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
            ambient_light: Arc::new(Vector3::new(0.0, 0.0, 0.0)),
        }
    }
}
