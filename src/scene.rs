use crate::camera::Camera;
use crate::light::Light;
use crate::primitive::*;
use nalgebra::{Matrix4, Point3, Vector3};
use rhai::{Engine, EvalAltResult};
use std::sync::Arc;
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

    pub fn from_rhai(script: &str) -> Result<Scene, Box<EvalAltResult>> {
        let mut engine = Engine::new();

        engine
            .register_type::<Vector3<f64>>()
            .register_fn("V", Vector3::<f64>::new);
        engine
            .register_type::<Point3<f64>>()
            .register_fn("P", Point3::<f64>::new);
        engine
            .register_type::<Scene>()
            .register_fn("Scene", Scene::empty)
            .register_fn("addNode", Scene::add_node)
            .register_fn("addLight", Scene::add_light);

        engine
            .register_type::<Node>()
            .register_fn("Node", Node::new)
            .register_fn("translate", Node::translate)
            .register_fn("rotate", Node::rotate)
            .register_fn("scale", Node::scale);
        engine
            .register_type::<Camera>()
            .register_fn("Camera", Camera::new_sizeless);
        engine
            .register_type::<Light>()
            .register_fn("Light", Light::new);
        engine
            .register_type::<Material>()
            .register_fn("Material", Material::new)
            .register_fn("MaterialRed", Material::red)
            .register_fn("MaterialBlue", Material::blue)
            .register_fn("MaterialGreen", Material::green)
            .register_fn("MaterialMagenta", Material::magenta)
            .register_fn("MaterialTurquoise", Material::turquoise);
        engine
            .register_type::<Sphere>()
            .register_fn("Sphere", Sphere::new)
            .register_fn("SphereUnit", Sphere::unit);
        engine
            .register_type::<Cube>()
            .register_fn("Cube", Cube::new)
            .register_fn("CubeUnit", Cube::unit);
        engine
            .register_type::<Cone>()
            .register_fn("Cone", Cone::new)
            .register_fn("ConeUnit", Cone::unit);
        engine
            .register_type::<Cylinder>()
            .register_fn("Cylinder", Cylinder::new);
        engine
            .register_type::<Circle>()
            .register_fn("Circle", Circle::new)
            .register_fn("CircleUnit", Circle::unit);
        engine
            .register_type::<Rectangle>()
            .register_fn("Rectangle", Rectangle::new)
            .register_fn("RectangleUnit", Rectangle::unit);
        engine
            .register_type::<SteinerSurface>()
            .register_fn("Steiner", SteinerSurface::new);
        engine
            .register_type::<Torus>()
            .register_fn("Torus", Torus::new);
        engine
            .register_type::<AdamShape>()
            .register_fn("Adam", AdamShape::new);
        engine
            .register_type::<AdamShape2>()
            .register_fn("Adam2", AdamShape2::new);
        engine
            .register_type::<AdamShape3>()
            .register_fn("Adam3", AdamShape3::new);

        let scene: Scene = engine.eval(script.into())?;
        Ok(scene)
    }
}
