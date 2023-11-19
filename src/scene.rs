use crate::camera::Camera;
use crate::light::Light;
use crate::primitive::Primitive;
use crate::primitive::*;
use nalgebra::{Matrix4, Point3, Vector3};
use rhai::{Engine, EvalAltResult};
use std::collections::HashMap;
use std::sync::Arc;

const LIGHT_AMBIENT: f32 = 0.2;

#[derive(Clone)]
pub struct Node {
    pub primitive: Arc<dyn Primitive>,
    pub model_transform: Matrix4<f32>,
}

impl Node {
    pub fn new(primitive: Arc<dyn Primitive>) -> Self {
        Node {
            primitive,
            model_transform: Matrix4::identity(),
        }
    }
    pub fn rotate(&mut self, roll: f32, pitch: f32, yaw: f32) {
        let roll = (roll as f64).to_radians() as f32;
        let pitch = (pitch as f64).to_radians() as f32;
        let yaw = (yaw as f64).to_radians() as f32;
        let rotation_matrix = Matrix4::from_euler_angles(roll, pitch, yaw);
        self.model_transform = rotation_matrix * self.model_transform;
    }
    pub fn translate(&mut self, translation: &Vector3<f32>) {
        let translation_matrix = Matrix4::new_translation(translation);
        self.model_transform = translation_matrix * self.model_transform;
    }
    pub fn scale(&mut self, scale: &Vector3<f32>) {
        let scale_matrix = Matrix4::new_nonuniform_scaling(scale);
        self.model_transform = scale_matrix * self.model_transform;
    }
}
#[derive(Clone)]
pub struct Scene {
    pub nodes: Vec<Node>,
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
    pub ambient_light: Vector3<f32>,
    pub camera: Camera,
}

impl Scene {
    // Creates an emptry scene
    pub fn empty() -> Self {
        Scene {
            nodes: Vec::new(),
            materials: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
            camera: Camera::new(
                Point3::new(0.0, 0.0, -10.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
                120.0,
                1.0,
            ),
            ambient_light: Vector3::new(LIGHT_AMBIENT, LIGHT_AMBIENT, LIGHT_AMBIENT),
        }
    }
    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    fn add_material(&mut self, material: Material) {
        self.materials.push(material);
    }
    fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    fn add_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }
    fn set_ambient(&mut self, intensity: Vector3<f32>) {
        self.ambient_light = intensity;
    }
    fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    fn get_camera(&self) -> &Camera {
        &self.camera
    }
    fn get_ambient(&self) -> Arc<Vector3<f32>> {
        Arc::new(self.ambient_light)
    }
    pub fn init(filename: &str) -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();

        engine
            .register_type::<Vector3<f32>>()
            .register_fn("V", Vector3::<f32>::new);
        engine
            .register_type::<Point3<f32>>()
            .register_fn("P", Point3::<f32>::new);
        engine
            .register_type::<Scene>()
            .register_fn("Scene", Scene::empty)
            .register_fn("addNode", Scene::add_node);
        engine
            .register_type::<Node>()
            .register_fn("Node", Node::new)
            .register_fn("translate", Node::translate)
            .register_fn("rotate", Node::rotate)
            .register_fn("scale", Node::scale);
        engine
            .register_type::<Camera>()
            .register_fn("Camera", Camera::new);
        engine
            .register_type::<Light>()
            .register_fn("Light", Light::new);
        engine
            .register_type::<Material>()
            .register_fn("Material", Material::new);
        engine
            .register_type::<Sphere>()
            .register_fn("Sphere", Sphere::new);

        engine.run_file(filename.into())?;
        Ok(())
    }
}
