use crate::camera::Camera;
use crate::light::Light;
use crate::primitive::*;
use nalgebra::{Matrix4, Vector3};
use std::sync::Arc;
#[derive(Clone)]
pub struct Node {
    pub primitive: Arc<dyn Primitive>,
    pub model: Matrix4<f64>,
    pub inv_model: Matrix4<f64>,
    pub viewmodel: Matrix4<f64>,
    pub inv_viewmodel: Matrix4<f64>,
}

impl Node {
    pub fn new(primitive: Arc<dyn Primitive>) -> Node {
        Node {
            primitive,
            model: Matrix4::identity(),
            inv_model: Matrix4::identity(),
            viewmodel: Matrix4::identity(),
            inv_viewmodel: Matrix4::identity(),
        }
    }
    pub fn rotate(&mut self, roll: f64, pitch: f64, yaw: f64) {
        let roll = roll.to_radians();
        let pitch = pitch.to_radians();
        let yaw = yaw.to_radians();
        let rotation_matrix = Matrix4::from_euler_angles(roll, pitch, yaw);
        self.model = rotation_matrix * self.model;
        self.inv_model = self.model.try_inverse().unwrap();
        self.viewmodel = rotation_matrix * self.viewmodel;
        self.inv_viewmodel = self.inv_viewmodel.try_inverse().unwrap();
    }
    pub fn translate(&mut self, translation: Vector3<f64>) {
        let translation_matrix = Matrix4::new_translation(&translation);
        self.model = translation_matrix * self.model;
        self.inv_model = self.model.try_inverse().unwrap();
        self.viewmodel = translation_matrix * self.viewmodel;
        self.inv_viewmodel = self.inv_viewmodel.try_inverse().unwrap();
    }
    pub fn scale(&mut self, scale: Vector3<f64>) {
        let scale_matrix = Matrix4::new_nonuniform_scaling(&scale);
        self.model = scale_matrix * self.model;
        self.inv_model = self.model.try_inverse().unwrap();
        self.viewmodel = scale_matrix * self.viewmodel;
        self.inv_viewmodel = self.inv_viewmodel.try_inverse().unwrap();
    }
    pub fn child(self, primitive: Arc<dyn Primitive>) -> Node {
        Node {
            primitive,
            model: self.model,
            inv_model: self.inv_model,
            viewmodel: self.model,
            inv_viewmodel: self.inv_model,
        }
    }
    pub fn compute(&mut self, view: &Matrix4<f64>, inv_view: &Matrix4<f64>) {
        self.viewmodel = view * self.model;
        self.inv_viewmodel = self.inv_model * inv_view;
    }
}

#[derive(Clone)]
pub struct Scene {
    pub nodes: Vec<Node>,
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
}

impl Scene {
    // Creates an emptry scene
    pub fn empty() -> Self {
        Scene {
            nodes: Vec::new(),
            materials: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
        }
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    pub fn add_material(&mut self, material: Material) {
        self.materials.push(material);
    }
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    pub fn add_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }
    pub fn compute(&mut self, view: &Matrix4<f64>, inv_view: &Matrix4<f64>) {
        for node in &mut self.nodes {
            node.compute(view, inv_view);
        }
    }
}
