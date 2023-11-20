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
        //All good
        let aspect = width as f64 / height as f64;
        let fovy_radians = (self.fovy as f64).to_radians();
        let fovh_radians = 2.0 * ((fovy_radians / 2.0).tan() * aspect).atan();
        // All good
        let view_direction = (self.target - self.eye).normalize();
        //All good
        let hor = view_direction.cross(&self.up).normalize(); // pointing right
        let vert = view_direction.cross(&hor).normalize(); // pointing up
                                                           //All good
        let h_width = 2.0 * (fovh_radians / 2.0).tan();
        let v_height = 2.0 * (fovy_radians / 2.0).tan();
        //All good
        let d_hor_vec = hor * (h_width / width as f64) as f32;
        let d_vert_vec = vert * (v_height / height as f64) as f32;

        let mut rays = Vec::with_capacity(width as usize * height as usize);

        for j in 0..height {
            for i in 0..width {
                let horizontal = (i as f32 - width as f32 / 2.0) * (d_hor_vec);
                let vertical = (j as f32 - height as f32 / 2.0) * (d_vert_vec);

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
        let view_direction = (self.target - self.eye).normalize(); // Normalize the view direction vector
        let hor = view_direction.cross(&self.up).normalize(); // pointing right
        let vert = view_direction.cross(&hor).normalize(); // pointing up
        let h_width = 2.0 * (fovh_radians / 2.0).tan();
        let v_height = 2.0 * (fovy_radians / 2.0).tan();
        let d_hor_vec = hor * (h_width / width as f64) as f32;
        let d_vert_vec = vert * (v_height / height as f64) as f32;

        // Calculate the offsets for the pixel's position on the image plane
        let horizontal = ((x as f32 / width as f32) - 0.5) * h_width as f32;
        let vertical = ((y as f32 / height as f32) - 0.5) * v_height as f32;

        // Calculate the ray direction by summing up the components
        let direction = Unit::new_normalize(
            view_direction + (horizontal * d_hor_vec) + (vertical * d_vert_vec),
        );

        Ray::new(self.eye, direction)
    }

    pub fn set_position(&mut self, eye: Point3<f32>) {
        self.eye = eye;
    }
}
