use crate::{camera::Camera, light::Light, material::*, node::*};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Scene {
    pub nodes: HashMap<String, Node>,
    pub materials: HashMap<String, Material>,
    pub lights: HashMap<String, Light>,
    pub cameras: HashMap<String, Camera>,
}

impl Scene {
    // Creates an emptry scene
    pub fn empty() -> Self {
        Scene {
            nodes: HashMap::new(),
            materials: HashMap::new(),
            lights: HashMap::new(),
            cameras: HashMap::new(),
        }
    }
    // Adds a node to the scene
    pub fn add_node(&mut self, label: String, node: Node) {
        self.nodes.insert(label, node);
    }
    // Adds a material to the scene
    pub fn add_material(&mut self, label: String, material: Material) {
        self.materials.insert(label, material);
    }
    // Adds a light to the scene
    pub fn add_light(&mut self, label: String, light: Light) {
        self.lights.insert(label, light);
    }
    // Adds a camera to the scene
    pub fn add_camera(&mut self, label: String, camera: Camera) {
        self.cameras.insert(label, camera);
    }
    // Compute all matricies for nodes
    pub fn compute(&mut self) {
        for (_, node) in &mut self.nodes {
            node.compute();
        }
    }
}
