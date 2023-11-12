use crate::EPSILON;
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

struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fovy: f32,
    aspect: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn new(
        eye: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        fovy: f32,
        aspect: f32,
    ) -> Self {
        let znear = EPSILON;
        let zfar = 1.0 / EPSILON;
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

    fn build_mvp_matrix(&self, model: Matrix4<f32>) -> Matrix4<f32> {
        let view = Matrix4::look_at_lh(&self.eye, &self.target, &self.up);
        let proj = Matrix4::new_perspective(self.aspect, self.fovy, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view * model;
    }
}
