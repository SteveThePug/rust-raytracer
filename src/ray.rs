use crate::{node::Node, scene::Scene};
use nalgebra::{Matrix4, Point3, Vector3};

// INTERSECTION -----------------------------------------------------------------
pub struct Intersection {
    // Information about an intersection
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,
}
impl Intersection {
    pub fn transform(&self, trans: &Matrix4<f64>, inv_trans: &Matrix4<f64>) -> Intersection {
        let point = trans.transform_point(&self.point);
        let normal = inv_trans.transpose().transform_vector(&self.normal);
        Intersection {
            point,
            normal,
            distance: self.distance,
        }
    }
}

// Ray struct represents a ray in 3D space with a starting point 'a' and a direction 'b'
#[derive(Clone)]
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
        let mut closest_distance = f64::MAX;
        let mut closest_intersect: Option<Intersection> = None;
        let mut closest_node = None;

        for (_, node) in &scene.nodes {
            if !node.active {
                continue;
            }
            // Transform ray into local model cordinates
            let ray = self.transform(&node.inv_model);
            // Check bounding box intersection
            if node.primitive.intersect_bounding_box(&ray) {
                // Check primitive intersection
                if let Some(intersect) = node.primitive.intersect_ray(&ray) {
                    // Check for closest distance
                    if intersect.distance < closest_distance {
                        closest_distance = intersect.distance;
                        closest_intersect = Some(intersect);
                        closest_node = Some(node);
                    }
                }
            }
        }

        //Shade the intersection point if there is one
        match closest_intersect {
            Some(intersect) => {
                //Inverse transform back to world coords
                let node = closest_node.unwrap();
                let intersect = intersect.transform(&node.model, &node.inv_model);
                Some(Ray::phong_shade_point(&scene, &self, &node, &intersect)) // If there is an intersection, shade it
            }
            None => None, // If there is no intersection, return None
        }
    }

    // Function to shade a point in the scene using Phong shading model
    pub fn phong_shade_point(
        scene: &Scene,
        ray: &Ray,
        node: &Node,
        intersect: &Intersection,
    ) -> Vector3<u8> {
        let point = &intersect.point;
        let normal = &intersect.normal;
        let incidence = &ray.b;

        let material = &node.material;
        let kd = &material.kd;
        let ks = &material.ks;
        let shininess = material.shininess;

        // Point to camera
        let to_camera = -incidence;

        // Compute the ambient light component and set it as base colour
        let mut colour = Vector3::zeros();

        for (_, light) in &scene.lights {
            if !light.active {
                continue;
            }
            if light.ambient {
                colour += light.colour;
                continue;
            }

            // Point to light
            let to_light = light.position - point;
            let light_distance = to_light.norm() as f32;
            let to_light = to_light.normalize();

            // let to_light_ray = Ray::new(point.clone() + 0.0001 * normal, to_light);
            // if to_light_ray.light_blocked(scene) {
            // continue;
            // }

            // Diffuse component
            let n_dot_l = normal.dot(&to_light).max(0.0) as f32;
            let diffuse = n_dot_l * kd;
            // Specular component
            let mut specular = Vector3::zeros();
            if n_dot_l > 0.0 {
                // Halfway vector.
                let h = to_camera + to_light.normalize();
                let n_dot_h = normal.dot(&h).max(0.0) as f32;
                specular = ks * n_dot_h.powf(shininess);
            }
            // Compute light falloff
            let falloff = 1.0
                / (1.0
                    + light.falloff[0]
                    + light.falloff[1] * light_distance
                    + light.falloff[2] * light_distance.powi(2));

            let light_intensity = light.colour.component_mul(&(diffuse + specular)) * falloff;
            colour += &light_intensity;
        }

        colour *= 255.0;
        let (r, g, b) = (colour.x as u8, colour.y as u8, colour.z as u8);
        Vector3::new(r, g, b)
    }

    pub fn light_blocked(&mut self, scene: &Scene) -> bool {
        for (_, node) in &scene.nodes {
            if !node.active {
                continue;
            }
            self.transform(&node.inv_model);
            if node.primitive.intersect_bounding_box(&self) {
                if node.primitive.intersect_ray(&self).is_some() {
                    return true;
                }
            }
        }
        false
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
