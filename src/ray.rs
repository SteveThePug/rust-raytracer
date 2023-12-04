use crate::{bvh::BVH, light::Light, node::Node, scene::Scene, state::RaytracingOption, EPSILON};
use nalgebra::{distance, Matrix3, Matrix4, Point3, Vector3};
use rand;

fn random_vec() -> Vector3<f64> {
    Vector3::new(rand::random(), rand::random(), rand::random())
}
fn random_unit_vec() -> Vector3<f64> {
    random_vec().normalize()
}

// INTERSECTION -----------------------------------------------------------------
pub struct Intersection {
    // Information about an intersection
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,
}
//Intersection point including point and normal
impl Intersection {
    pub fn transform(&mut self, trans: &Matrix4<f64>, inv_trans: &Matrix4<f64>) -> Intersection {
        Intersection {
            point: trans.transform_point(&self.point),
            normal: inv_trans.transpose().transform_vector(&self.normal),
            distance: self.distance,
        }
    }
    pub fn transform_mut(&mut self, trans: &Matrix4<f64>, inv_transpose: &Matrix3<f64>) {
        self.point = trans.transform_point(&self.point);
        self.normal = inv_transpose * self.normal;
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
    // Return a transformed version of the ray
    pub fn transform(&self, trans: &Matrix4<f64>) -> Ray {
        Ray {
            a: trans.transform_point(&self.a),
            b: trans.transform_vector(&self.b),
        }
    }
    //Transform mutably
    pub fn transform_mut(&mut self, trans: &Matrix4<f64>) {
        self.a = trans.transform_point(&self.a);
        self.b = trans.transform_vector(&self.b);
    }
    //This function will determine if the ray hits an object in the scene
    //This is not optimised as it does not include bounding boxes
    pub fn hit_scene(ray: &Ray, scene: &Scene) -> bool {
        for (_, node) in &scene.nodes {
            if !node.active {
                continue;
            }
            // Transform ray into local model cordinates
            if node.intersect_ray(&ray).is_some() {
                return true;
            }
        }
        false
    }
    //This function find the closest intersection point of a ray with an object in the scene
    //Also not optimised, as it does not include bounding boxes
    pub fn closest_intersect<'a>(
        ray: &'a Ray,
        scene: &'a Scene,
    ) -> Option<(&'a Node, Intersection)> {
        let mut closest_distance = f64::MAX;
        let mut closest_intersect: Option<(&Node, Intersection)> = None;
        let ray_a = ray.a;
        for (_, node) in &scene.nodes {
            //position of ray in world coords
            if !node.active {
                continue;
            }

            if node.aabb.intersect_ray(&ray) {
                //Check node intersection
                if let Some(intersect) = node.intersect_ray(&ray) {
                    // Check for closest distance by converting to world coords
                    let distance = distance(&ray_a, &intersect.point);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_intersect = Some((node, intersect));
                    }
                }
            }
        }
        closest_intersect
    }
    // This function takes a scene and returns the color of the point where the ray intersects the scene
    pub fn shade_ray(
        &self,
        scene: &Scene,
        depth: u8,
        options: &RaytracingOption,
        sbvh: &Option<BVH>,
    ) -> Option<Vector3<f32>> {
        //If we have exceeded depth then return
        if depth == options.ray_depth {
            return None;
        }
        match sbvh {
            //We have a bvh so use bvh traversal
            Some(bvh) => {
                //Intersect the scene with the bvh
                if let Some((node, intersect)) = bvh.traverse(self, 0) {
                    return Some(Ray::phong_shade_point(
                        &scene, &self, &node, &intersect, depth, options, sbvh,
                    ));
                }
                return None;
            }
            //We dont have a bvh so use generic algorithm
            None => {
                //No BVH given so intersect normally
                match Ray::closest_intersect(self, scene) {
                    Some((node, intersect)) => {
                        Some(Ray::phong_shade_point(
                            &scene, &self, &node, &intersect, depth, options, sbvh,
                        )) // If there is an intersection, shade it
                    }
                    None => None, // If there is no intersection, return None
                }
            }
        }
    }

    // Function to shade a point in the scene using Phong shading model
    pub fn phong_shade_point(
        scene: &Scene,
        ray: &Ray,
        node: &Node,
        intersect: &Intersection,
        depth: u8,
        options: &RaytracingOption,
        bvh: &Option<BVH>,
    ) -> Vector3<f32> {
        let normal = &intersect.normal;
        let point = &intersect.point;
        let incidence = &ray.b;
        let material = &node.material;

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

            //Niave Shadows
            if options.shadows {
                let to_light_ray = Ray::new(*point, to_light);
                if to_light_ray.light_blocked(scene, light, bvh) {
                    continue;
                }
            }

            let n_dot_l = normal.dot(&to_light).max(0.0) as f32;

            //Reflected component
            let mut reflect = Vector3::zeros();
            if options.reflect {
                let reflect_dir = incidence - 2.0 * incidence.dot(&normal) * normal;
                let reflect_ray = Ray::new(*point, reflect_dir);
                if let Some(col) = reflect_ray.shade_ray(scene, depth + 1, options, bvh) {
                    reflect += col.component_mul(&material.kr)
                }
            }

            //Diffuse component (Lambertian)
            let mut diffuse = Vector3::zeros();
            if options.diffuse {
                diffuse += material.kd * n_dot_l;
                for _ in 0..options.diffuse_rays {
                    let diffuse_dir = random_unit_vec();
                    let diffuse_ray = Ray::new(point.clone(), diffuse_dir + normal);
                    if let Some(col) = diffuse_ray.shade_ray(scene, depth + 1, options, bvh) {
                        diffuse += col * options.diffuse_coefficient;
                    }
                }
            }

            //Specular component
            let mut specular = Vector3::zeros();
            if options.specular {
                if n_dot_l > 0.0 {
                    let h = (to_light - incidence).normalize();
                    let n_dot_h = normal.dot(&h).max(0.0) as f32;
                    specular = material.ks * n_dot_h.powf(material.shininess);
                }
            }

            //Falloff
            let mut falloff = 1.0;
            if options.falloff {
                falloff = 1.0
                    / ((1.0 + light.falloff[0])
                        + light.falloff[1] * light_distance
                        + light.falloff[2] * light_distance * light_distance);
            }

            let intensity = light.colour.component_mul(&(diffuse + reflect + specular)) * falloff;
            colour += &intensity;
        }

        colour
    }

    pub fn light_blocked(&self, scene: &Scene, light: &Light, bvh: &Option<BVH>) -> bool {
        let light_distance = distance(&self.a, &light.position);
        match bvh {
            Some(bvh) => {
                //We have a bvh so use bvh traversal
                for (_, node) in &scene.nodes {
                    if !node.active {
                        continue;
                    }
                    match bvh.traverse(self, 0) {
                        Some((_, intersect)) => {
                            if intersect.distance < light_distance + EPSILON {
                                return true;
                            }
                        }
                        None => continue,
                    }
                }
                return false;
            }
            None => {
                for (_, node) in &scene.nodes {
                    if !node.active {
                        continue;
                    }
                    if node.aabb.intersect_ray(self) {
                        match node.intersect_ray(self) {
                            Some(intersect) => {
                                if intersect.distance < light_distance {
                                    return true;
                                }
                            }
                            None => continue,
                        }
                    }
                }
            }
        }
        return false;
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
        let zv = (target - eye).normalize();
        let xv = zv.cross(&up).normalize();
        let yv = xv.cross(&zv).normalize();
        // ☐ height and width of projection
        let vheight = 2.0 * (fovy_radians / 2.0).tan();
        let vwidth = 2.0 * (fovh_radians / 2.0).tan();
        // Increment of right and up per pixel
        let dy = vheight / height;
        let dx = vwidth / width;
        let dxv = dx * xv;
        let dyv = dy * yv;
        // Half the width for later calculation
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        // Array of rays
        let mut rays = Vec::with_capacity(width as usize * height as usize);
        // Iterate column by row
        for y in 0..height as u32 {
            for x in 0..width as u32 {
                let x = (x as f64) - half_width;
                let y = half_height - (y as f64);

                let horizontal = x * &dxv;
                let vertical = y * &dyv;
                let direction = (zv + horizontal + vertical).normalize();
                let ray = Ray::new(eye.clone(), direction);
                rays.push(ray);
            }
        }
        rays
    }
}
