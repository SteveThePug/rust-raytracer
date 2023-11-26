use nalgebra::{Matrix4, Point3, Vector3};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Camera {
    eye: Point3<f64>,
    target: Point3<f64>,
    up: Vector3<f64>,
    pub view: Matrix4<f64>,
    pub inv_view: Matrix4<f64>,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(eye: Point3<f64>, target: Point3<f64>, up: Vector3<f64>) -> Self {
        let view = Matrix4::look_at_lh(&eye, &target, &up);
        let inv_view = view.try_inverse().unwrap();
        Camera {
            eye,
            target,
            up,
            view,
            inv_view,
        }
    }

    pub fn unit() -> Self {
        let eye = Point3::new(0.0, 0.0, 1.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        Camera::new(eye, target, up)
    }

    pub fn set_eye(&mut self, new_eye: Point3<f64>) {
        self.eye = new_eye;
        self.recalculate_matrix();
    }

    pub fn set_target(&mut self, new_target: Point3<f64>) {
        self.target = new_target;
        self.recalculate_matrix();
    }

    pub fn set_up(&mut self, new_up: Vector3<f64>) {
        self.up = new_up;
        self.recalculate_matrix();
    }

    fn recalculate_matrix(&mut self) {
        self.view = Matrix4::look_at_lh(&self.eye, &self.target, &self.up);
        self.inv_view = self.view.try_inverse().unwrap();
    }
}
