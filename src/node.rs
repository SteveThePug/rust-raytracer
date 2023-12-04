use crate::{
    bvh::AABB,
    material::Material,
    primitive::{self, *},
    ray::{Intersection, Ray},
};
use nalgebra::{Matrix4, Vector3};
use std::sync::Arc;

#[derive(Clone)]
pub struct Node {
    //Primitive
    pub primitive: Arc<dyn Primitive>,
    pub material: Material,
    pub aabb: AABB,
    //Transformations
    pub rotation: [f64; 3],
    pub scale: [f64; 3],
    pub translation: [f64; 3],
    //Model matricies
    pub model: Matrix4<f64>,
    pub inv_model: Matrix4<f64>,
    //If the node is active
    pub active: bool,
}

impl Node {
    //New node with no transformations
    pub fn new(primitive: Arc<dyn Primitive>, material: Material) -> Node {
        let aabb = primitive.get_aabb();
        Node {
            primitive,
            material,
            aabb,
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            translation: [0.0, 0.0, 0.0],
            model: Matrix4::identity(),
            inv_model: Matrix4::identity(),

            active: true,
        }
    }
    //New node with parent transformations
    pub fn child(self, primitive: Arc<dyn Primitive>) -> Node {
        let mut child = self.clone();
        child.primitive = primitive;
        child
    }
    //Toggle is a mesh is visible or not
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    //Rotate a mesh by adding to its rotation
    pub fn rotate(&mut self, roll: f64, pitch: f64, yaw: f64) {
        //Convert to radians
        let roll = roll.to_radians();
        // Convert pitch and yaw to radians
        let pitch = pitch.to_radians();
        let yaw = yaw.to_radians();

        // Add the roll, pitch, and yaw to the current rotation
        self.rotation[0] += roll;
        self.rotation[1] += pitch;
        self.rotation[2] += yaw;

        // Recompute the model and inverse model matrices
        self.compute();
    }
    // Translate a mesh by adding to its current position
    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        self.translation[0] += x;
        self.translation[1] += y;
        self.translation[2] += z;

        // Recompute the model and inverse model matrices
        self.compute();
    }
    // Scale a mesh by adding to its current scale
    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        self.scale[0] += x;
        self.scale[1] += y;
        self.scale[2] += z;

        // Recompute the model and inverse model matrices
        self.compute();
    }
    // This function computes the model and inverse model matrices
    pub fn compute(&mut self) {
        //Translation matrix
        let translation = Vector3::from_row_slice(&self.translation);
        let translation_matrix = Matrix4::new_translation(&translation);
        // Scale matrix
        let scale = &Vector3::from_row_slice(&self.scale);
        let scale_matrix = Matrix4::new_nonuniform_scaling(&scale);
        // Rotation matrix
        let (roll, pitch, yaw) = (self.rotation[0], self.rotation[1], self.rotation[2]);
        let rotation_matrix = Matrix4::from_euler_angles(roll, pitch, yaw);
        // Compute the model matrix by combining the translation, rotation, and scale matrices
        self.model = (translation_matrix * rotation_matrix * scale_matrix).cast();
        // Compute the inverse model matrix by inverting the model matrix
        self.inv_model = self.model.try_inverse().unwrap();
        self.aabb.transform_mut(&self.model);
    }
    // Intersection of a ray, will convert to model coords and check
    pub fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let ray = ray.transform(&self.inv_model); //Transform from world coordinates
        if let Some(mut intersect) = self.primitive.intersect_ray(&ray) {
            intersect.transform_mut(&self.model, &self.inv_model); //Transform to world coords
            return Some(intersect);
        }
        return None;
    }
    //Gets the bounding box in world coords
    pub fn get_world_aabb(&self) -> AABB {
        return self.aabb.clone();
    }
}
