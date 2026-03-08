use nalgebra::{Matrix4, Point3, Vector3};

/// Annotate the Camera struct
#[derive(Clone)]
pub struct Camera {
    pub eye: Point3<f64>,
    pub target: Point3<f64>,
    pub up: Vector3<f64>,
    pub _view: Matrix4<f64>,
    pub _inv_view: Matrix4<f64>,
}

#[allow(dead_code)]
impl Camera {
    /// Create a new camera with the given eye, target, and up vectors
    pub fn new(eye: Point3<f64>, target: Point3<f64>, up: Vector3<f64>) -> Self {
        let view = Matrix4::look_at_lh(&eye, &target, &up);
        let inv_view = view.try_inverse().unwrap();
        Camera {
            eye,
            target,
            up,
            _view: view,
            _inv_view: inv_view,
        }
    }

    /// Create a unit camera with default parameters
    pub fn unit() -> Self {
        let eye = Point3::new(2.0, 2.0, 2.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        Camera::new(eye, target, up)
    }

    /// Set the position of the camera's eye
    pub fn set_eye(&mut self, new_eye: Point3<f64>) {
        self.eye = new_eye;
        self.recalculate_matrix();
    }

    /// Set the position the camera is looking at
    pub fn set_target(&mut self, new_target: Point3<f64>) {
        self.target = new_target;
        self.recalculate_matrix();
    }

    /// Set the up vector of the camera
    pub fn set_up(&mut self, new_up: Vector3<f64>) {
        self.up = new_up;
        self.recalculate_matrix();
    }

    /// Get the forward direction vector (from eye toward target)
    pub fn forward(&self) -> Vector3<f64> {
        (self.target - self.eye).normalize()
    }

    /// Get the right direction vector
    pub fn right(&self) -> Vector3<f64> {
        self.forward().cross(&self.up).normalize()
    }

    /// Move the camera forward/backward along its view direction (moves both eye and target)
    pub fn move_forward(&mut self, amount: f64) {
        let dir = self.forward() * amount;
        self.eye += dir;
        self.target += dir;
        self.recalculate_matrix();
    }

    /// Strafe the camera left/right (moves both eye and target)
    pub fn move_right(&mut self, amount: f64) {
        let dir = self.right() * amount;
        self.eye += dir;
        self.target += dir;
        self.recalculate_matrix();
    }

    /// Move the camera up/down along the up vector (moves both eye and target)
    pub fn move_up(&mut self, amount: f64) {
        let dir = self.up.normalize() * amount;
        self.eye += dir;
        self.target += dir;
        self.recalculate_matrix();
    }

    /// Orbit the camera around the target point by yaw (horizontal) and pitch (vertical) angles in radians
    pub fn orbit(&mut self, yaw: f64, pitch: f64) {
        let offset = self.eye - self.target;
        let radius = offset.norm();

        // Current spherical angles
        let current_pitch = (offset.y / radius).asin();
        let current_yaw = offset.z.atan2(offset.x);

        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(
            -std::f64::consts::FRAC_PI_2 + 0.01,
            std::f64::consts::FRAC_PI_2 - 0.01,
        );

        // Convert back to cartesian
        let new_offset = Vector3::new(
            radius * new_pitch.cos() * new_yaw.cos(),
            radius * new_pitch.sin(),
            radius * new_pitch.cos() * new_yaw.sin(),
        );

        self.eye = self.target + new_offset;
        self.recalculate_matrix();
    }

    /// Recalculate the view and inverse view matrices based on the current eye, target, and up vectors
    fn recalculate_matrix(&mut self) {
        self._view = Matrix4::look_at_lh(&self.eye, &self.target, &self.up);
        self._inv_view = self._view.try_inverse().unwrap();
    }
}
