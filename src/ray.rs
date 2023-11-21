use crate::{
    primitive::Intersection,
    raytracer::phong_shade_point,
    scene::{Node, Scene},
    EPSILON, INFINITY,
};
use nalgebra::{Point3, Unit, Vector3};

pub struct Ray {
    pub a: Point3<f32>,
    pub b: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn new(a: Point3<f32>, b: Unit<Vector3<f32>>) -> Ray {
        Ray { a, b }
    }
    pub fn at_t(&self, t: f32) -> Point3<f32> {
        self.a + self.b.into_inner() * (t + EPSILON)
    }
    //Shade a single ray
    pub fn shade_ray(&self, scene: &Scene) -> Option<Vector3<u8>> {
        let intersect = self.get_closest_intersection(&scene.nodes);
        match intersect {
            Some(intersect) => Some(phong_shade_point(&scene, &intersect)),
            None => None,
        }
    }

    // Find the closest intersection, given a ray in world coordinates
    pub fn get_closest_intersection(&self, nodes: &Vec<Node>) -> Option<Intersection> {
        let mut closest_distance = INFINITY;
        let mut closest_intersect: Option<Intersection> = None;

        for node in nodes {
            let primitive = node.primitive.clone();

            if let Some(intersect) = primitive.intersect_ray(self) {
                if intersect.distance < closest_distance {
                    closest_distance = intersect.distance;
                    closest_intersect = Some(intersect);
                }
            }
        }

        closest_intersect
    }
}
