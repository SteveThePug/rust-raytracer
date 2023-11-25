use crate::ray::Ray;
use crate::{EPSILON, INFINITY};
use nalgebra::{Matrix4, Perspective3, Point3, Unit, Vector3};

const ZNEAR: f32 = EPSILON;
const ZFAR: f32 = INFINITY;

#[derive(Clone)]
pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fovy: f32,
    aspect: f32,
    matrix: Matrix4<f32>,
    inverse: Matrix4<f32>,
}

impl Camera {
    pub fn new(
        eye: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        fovy: f32,
        aspect: f32,
    ) -> Self {
        let (matrix, inverse) = Camera::build_matrix_and_inverse(&eye, &target, &up, fovy, aspect);
        Camera {
            eye,
            target,
            up,
            fovy,
            matrix,
            inverse,
            aspect,
        }
    }

    pub fn build_matrix_and_inverse(
        eye: &Point3<f32>,
        target: &Point3<f32>,
        up: &Vector3<f32>,
        fovy: f32,
        aspect: f32,
    ) -> (Matrix4<f32>, Matrix4<f32>) {
        let view = Matrix4::look_at_lh(eye, target, up);
        let proj = Perspective3::new(aspect, fovy, ZNEAR, ZFAR);
        let matrix = proj.as_matrix() * view;
        let inverse = view.try_inverse().expect("No view") * proj.inverse();
        (matrix, inverse)
    }

    pub fn cast_rays(&self, width: u32, height: u32) -> Vec<Ray> {
        let aspect = width as f64 / height as f64;
        let fovy_radians = (self.fovy as f64).to_radians();
        let fovh_radians = 2.0 * ((fovy_radians / 2.0).tan() * aspect).atan();
        let view_direction = (self.target - self.eye).normalize();
        let hor = (view_direction.cross(&self.up)).normalize();
        let vert = (view_direction.cross(&hor)).normalize();
        let h_width = 2.0 * (fovh_radians / 2.0).tan();
        let v_height = 2.0 * (fovy_radians / 2.0).tan();
        //All good
        let d_hor_vec = hor * (h_width / width as f64) as f32;
        let d_vert_vec = vert * (v_height / height as f64) as f32;

        let mut rays = Vec::with_capacity(width as usize * height as usize);

        let half_w = width as i32 / 2;
        let half_h = height as i32 / 2;

        for j in 0..height as i32 {
            for i in 0..width as i32 {
                let horizontal = (i - half_w) as f32 * (d_hor_vec);
                let vertical = (-j + half_h) as f32 * (d_vert_vec);

                let direction = view_direction + horizontal + vertical;
                let ray = Ray::new(self.eye, Unit::new_normalize(direction));
                rays.push(ray);
            }
        }
        rays
    }

    pub fn cast_ray(&self, width: u32, height: u32, x: u32, y: u32) -> Ray {
        let aspect = width as f64 / height as f64;
        let fovy_radians = (self.fovy as f64).to_radians();
        let fovh_radians = 2.0 * ((fovy_radians / 2.0).tan() * aspect).atan();
        let view_direction = (self.target - self.eye).normalize();
        let hor = (view_direction.cross(&self.up)).normalize();
        let vert = (view_direction.cross(&hor)).normalize();
        let h_width = 2.0 * (fovh_radians / 2.0).tan();
        let v_height = 2.0 * (fovy_radians / 2.0).tan();
        //All good
        let d_hor_vec = hor * (h_width / width as f64) as f32;
        let d_vert_vec = vert * (v_height / height as f64) as f32;

        let half_w = width as i32 / 2;
        let half_h = height as i32 / 2;

        let horizontal = (x as i32 - half_w) as f32 * (d_hor_vec);
        let vertical = (-(y as i32) + half_h) as f32 * (d_vert_vec);

        let direction = view_direction + horizontal + vertical;
        let ray = Ray::new(self.eye, Unit::new_normalize(direction));

        Ray::new(self.eye, Unit::new_normalize(direction))
    }

    pub fn set_position(&mut self, eye: Point3<f32>) {
        self.eye = eye;
    }
}
