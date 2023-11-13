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

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_lh(&self.eye, &self.target, &self.up);
        let proj = Matrix4::new_perspective(self.aspect, self.fovy, self.znear, self.zfar);
        proj * view
    }
    pub fn build_inverse_view_projection_matrix(&self) -> Matrix4<f32> {
        let view_proj = self.build_view_projection_matrix();
        view_proj.try_inverse().expect("Cannot invert!")
    }
    pub fn cast_rays(&self, width: usize, height: usize) -> Vec<Ray> {
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
}
