use crate::{
    primitive::Intersection,
    raytracer::phong_shade_point,
    scene::{Node, Scene},
    EPSILON, INFINITY,
};
use nalgebra::{Point3, Unit, Vector3};

#[derive(Clone)]
pub struct Ray {
    pub a: Point3<f64>,
    pub b: Unit<Vector3<f64>>,
}

impl Ray {
    pub fn new(a: Point3<f64>, b: Unit<Vector3<f64>>) -> Ray {
        Ray { a, b }
    }
    pub fn unit() -> Ray {
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0));
        Ray { a, b }
    }
    pub fn at_t(&self, t: f64) -> Point3<f64> {
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
            let trans = node.trans;
            let inv_trans = node.inv_trans;

            if let Some(intersect) = primitive.intersect_ray(self) {
                if intersect.distance < closest_distance {
                    closest_distance = intersect.distance;
                    closest_intersect = Some(intersect);
                }
            }
        }

        closest_intersect
    }

    pub fn cast_rays(fovy: f64, width: u32, height: u32) -> Vec<Ray> {
        let aspect = width as f64 / height as f64;
        let fovy_radians = fovy.to_radians();
        //Verify this part later
        let dir = Vector3::new(0.0, 0.0, 1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let hor = Vector3::new(1.0, 0.0, 0.0);
        let half_height = fovy_radians.tan();
        let half_width = aspect * half_height;

        let d_hor_vec = hor * (2.0 * half_width / width as f64) as f64;
        let d_vert_vec = up * (2.0 * half_height / height as f64) as f64;

        //All good

        let mut rays = Vec::with_capacity(width as usize * height as usize);

        for j in 0..height as i32 {
            for i in 0..width as i32 {
                let horizontal = (i - half_width as i32) as f64 * (d_hor_vec);
                let vertical = (-j + half_height as i32) as f64 * (d_vert_vec);

                let direction = dir + horizontal + vertical;
                let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Unit::new_normalize(direction));
                rays.push(ray);
            }
        }
        rays
    }
}
