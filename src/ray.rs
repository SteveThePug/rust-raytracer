use crate::{
    primitive::Intersection,
    raytracer::phong_shade_point,
    scene::{Node, Scene},
    INFINITY,
};
use nalgebra::{Matrix4, Point3, Vector3};

#[derive(Clone)]
pub struct Ray {
    pub a: Point3<f64>,
    pub b: Vector3<f64>,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(a: Point3<f64>, b: Vector3<f64>) -> Ray {
        Ray {
            a,
            b: b.normalize(),
        }
    }
    pub fn unit() -> Ray {
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        Ray { a, b }
    }
    pub fn at_t(&self, t: f64) -> Point3<f64> {
        self.a + self.b * t
    }
    //Shade a single ray
    pub fn shade_ray(&self, scene: &Scene) -> Option<Vector3<u8>> {
        let intersect = self.get_closest_intersection(&scene.nodes);

        match intersect {
            Some(intersect) => Some(phong_shade_point(&scene, &intersect)),
            None => None,
        }
    }

    // Find the closest intersection
    pub fn get_closest_intersection(&self, nodes: &Vec<Node>) -> Option<Intersection> {
        let mut closest_distance = INFINITY;
        let mut closest_intersect: Option<Intersection> = None;

        for node in nodes {
            let primitive = node.primitive.clone();

            //Transform ray from view coords
            let ray = self.transform(&node.inv_viewmodel);

            if primitive.intersect_bounding_box(&ray).is_some() {
                if let Some(intersect) = primitive.intersect_ray(&ray) {
                    if intersect.distance < closest_distance {
                        closest_distance = intersect.distance;
                        //Convert back to world coords
                        let intersect = intersect.transform(&node.model, &node.inv_model);

                        closest_intersect = Some(intersect);
                    }
                }
            }
        }

        closest_intersect
    }

    pub fn transform(&self, trans: &Matrix4<f64>) -> Ray {
        Ray {
            a: trans.transform_point(&self.a),
            b: trans.transform_vector(&self.b),
        }
    }

    pub fn cast_rays(fovy: f64, width: u32, height: u32) -> Vec<Ray> {
        let aspect = width as f64 / height as f64;
        let fovy_radians = fovy.to_radians();
        let fovh_radians = 2.0 * ((fovy_radians / 2.0).tan() * aspect).atan();

        let dir = Vector3::new(0.0, 0.0, 1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let hor = Vector3::new(1.0, 0.0, 0.0);
        let vheight = 2.0 * (fovy_radians / 2.0).tan();
        let vwidth = 2.0 * (fovh_radians / 2.0).tan();

        let d_hor_vec = hor * (vwidth / width as f64) as f64;
        let d_vert_vec = up * (vheight / height as f64) as f64;

        let half_width = width / 2;
        let half_height = height / 2;

        let mut rays = Vec::with_capacity(width as usize * height as usize);

        for j in 0..height as i32 {
            for i in 0..width as i32 {
                let x = i - half_width as i32;
                let y = -j + half_height as i32;
                let horizontal = x as f64 * d_hor_vec;
                let vertical = y as f64 * (d_vert_vec);
                let direction = dir + horizontal + vertical;
                let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), direction);
                rays.push(ray);
            }
        }
        rays
    }
}
