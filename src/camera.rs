use crate::ray::Ray;
use crate::{EPSILON, INFINITY};
use nalgebra as nm;
use nalgebra::Matrix4;
use nalgebra::Point3;
use nalgebra::Vector3;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fovy: f32,
    aspect: f32,
    znear: f32,
    zfar: f32,
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
        let znear = EPSILON;
        let zfar = INFINITY;
        let matrix = self.build_view_projection_matrix(eye, target, up, aspect, fovy, znear, zfar);
        let inverse = self.build_inverse_view_projection_matrix(eye, target, up, aspect, fovy, znear, zfar);
        Camera {
            eye,
            target,
            up,
            fovy,
            aspect,
            znear,
            zfar,
        }
    }

    pub fn build_view_projection_matrix(eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>, aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Matrix4<f32> {
        let view = Matrix4::look_at_lh(eye, target, up);
        let proj = Matrix4::new_perspective(aspect, fovy,znear, zfar);
        proj * view
    }
    pub fn build_inverse_view_projection_matrix(eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>, aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Matrix4<f32> {
        let view_proj = self.build_view_projection_matrix(eye, target, up, aspect, fovy, znear, zfar);
        view_proj.try_inverse().expect("Cannot invert!")
    }
    pub fn cast_rays(&self, width: u32, height: u32) -> Vec<Ray> {
        let inverse_matrix = self.build_inverse_view_projection_matrix();

        let dx = 2.0 / width as f32;
        let dy = 2.0 / height as f32;

        let mut rays = Vec::with_capacity(width as usize * height as usize);

        for i in 0..width {
            for j in 0..height {
                let x = -1.0 + i as f32 * dx;
                let y = 1.0 - j as f32 * dy;

                let a = inverse_matrix.transform_point(&Point3::new(x, y, -1.0));
                let b = inverse_matrix.transform_vector(&Vector3::new(0.0, 0.0, 1.0));

                let ray = Ray { a, b };
                rays.push(ray);
            }
        }
        rays
    }
    pub fn cast_ray(&self, width: u32, height: u32, x: u32, y: u32) -> Ray {
        let dx = 2.0 / width as f32;
        let dy = 2.0 / height as f32;
    }
}
