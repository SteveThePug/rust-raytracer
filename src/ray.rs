use crate::{
    primitive::Intersection,
    raytracer::phong_shade_point,
    scene::{Node, Scene},
};
use nalgebra::{Matrix4, Point3, Vector3};

#[derive(Clone)]
// Ray struct represents a ray in 3D space with a starting point 'a' and a direction 'b'
pub struct Ray {
    pub a: Point3<f64>,
    pub b: Vector3<f64>,
}

#[allow(dead_code)]
impl Ray {
    //Create a new ray with a normalized direction
    pub fn new(a: Point3<f64>, b: Vector3<f64>) -> Ray {
        Ray {
            a,
            b: b.normalize(),
        }
    }
    // The starting point is the origin and the direction is negative z-axis
    pub fn unit() -> Ray {
        let a = Point3::origin();
        let b = -Vector3::z();
        Ray { a, b }
    }
    //Return the point at distance t along the ray
    pub fn at_t(&self, t: f64) -> Point3<f64> {
        self.a + self.b * t
    }
    // This function takes a scene and returns the color of the point where the ray intersects the scene
    pub fn shade_ray(&self, scene: &Scene) -> Option<Vector3<u8>> {
        //Get the closest intersection of the ray with the scene
        let intersect = self.get_closest_intersection(&scene.nodes);
        //Shade the intersection point if there is one
        match intersect {
            Some(intersect) => Some(phong_shade_point(&scene, &intersect)), // If there is an intersection, shade it
            None => None, // If there is no intersection, return None
        }
    }

    // Find the closest intersection
    pub fn get_closest_intersection(&self, nodes: &Vec<Node>) -> Option<Intersection> {
        //Assign no intersection
        let mut closest_distance = f64::MAX;
        let mut closest_intersect: Option<Intersection> = None;

        for node in nodes {
            // Clone arc to primitive
            let primitive = node.primitive.clone();
            // Transform ray into local model cordinates
            let ray = self.transform(&node.inv_model);
            // Check bounding box intersection
            if primitive.intersect_bounding_box(&ray).is_some() {
                // Check primitive intersection
                if let Some(intersect) = primitive.intersect_ray(&ray) {
                    // Check for closest distance
                    if intersect.distance < closest_distance {
                        closest_distance = intersect.distance;
                        //Convert back to world coords
                        let intersect = intersect.transform(&node.model, &node.inv_model);
                        closest_intersect = Some(intersect);
                    }
                }
            }
        }
        //Return None if we find no intersection, some if we do find one
        closest_intersect
    }
    // Return a transformed version of the ray
    pub fn transform(&self, trans: &Matrix4<f64>) -> Ray {
        Ray {
            a: trans.transform_point(&self.a),
            b: trans.transform_vector(&self.b),
        }
    }
    //Cast a set of rays
    pub fn cast_rays(
        eye: &Point3<f64>,
        target: &Point3<f64>,
        up: &Vector3<f64>,
        fovy: f64,
        width: u32,
        height: u32,
    ) -> Vec<Ray> {
        //Aspect ratio calculation
        let (width, height) = (width as f64, height as f64);
        let aspect = width / height;
        //X and Y fov calculations
        let fovy_radians = fovy.to_radians();
        let fovh_radians = 2.0 * ((fovy_radians / 2.0).tan() * aspect).atan();
        // Vectors pointing forward, right and up
        let forward = (target - eye).normalize();
        let right = forward.cross(&up).normalize();
        let up = right.cross(&forward).normalize();
        // ☐ height and width of projection
        let vheight = 2.0 * (fovy_radians / 2.0).tan();
        let vwidth = 2.0 * (fovh_radians / 2.0).tan();
        // Increment of right and up per pixel
        let d_hor_vec = right * (vwidth / width);
        let d_vert_vec = up * (vheight / height);
        // Half the width for later calculation
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        // Array of rays
        let mut rays = Vec::with_capacity(width as usize * height as usize);
        // Iterate column by row
        for row in 0..height as u32 {
            for column in 0..width as u32 {
                let x = (column as f64) - half_width;
                let y = half_height - (row as f64);

                let horizontal = x * &d_hor_vec;
                let vertical = y * &d_vert_vec;
                let direction = (forward + horizontal + vertical).normalize();
                let ray = Ray::new(eye.clone(), direction);
                rays.push(ray);
            }
        }
        rays
    }
}
