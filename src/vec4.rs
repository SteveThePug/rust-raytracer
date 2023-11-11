use std::fmt;
use std::ops;
// Vectors will have 4 components
// Color: r,b,g,a
// Vector: x,y,z,0
// Position: x,y,z,1

#[derive(Copy, Clone)]
pub struct Vec4 {
    x: [f32; 4],
}

impl Vec4 {
    //Initialise a vector with 4 components
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x: [x, y, z, w] }
    }
    //Create a unit vector
    pub fn unit() -> Vec4 {
        Vec4 {
            x: [1.0, 1.0, 1.0, 1.0],
        }
    }
    //Create a vector will all entries equal to x
    pub fn with_value(x: f32) -> Vec4 {
        Vec4 { x: [x, x, x, x] }
    }
    //Dot product
    pub fn dot(&self, other: &Vec4) -> f32 {
        self.x[0] * other.x[0]
            + self.x[1] * other.x[1]
            + self.x[2] * other.x[2]
            + self.x[3] * other.x[3]
    }
    //Cross product
    pub fn cross(&self, other: &Vec4) -> Vec4 {
        Vec4 {
            x: [
                self.x[1] * other.x[2] - self.x[2] * other.x[1],
                self.x[2] * other.x[0] - self.x[0] * other.x[2],
                self.x[0] * other.x[1] - self.x[1] * other.x[0],
                0.0,
            ],
        }
    }
    //Normalise the vector
    pub fn normalise(&mut self) {
        let magnitude = self.dot(self).sqrt();
        if magnitude != 0.0 {
            *self /= magnitude;
        }
    }
}
// METHODS TO ADD---------------------------
// + Vec4
impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] + other.x[0],
                self.x[1] + other.x[1],
                self.x[2] + other.x[2],
                self.x[3] + other.x[3],
            ],
        }
    }
}
// + f32
impl ops::Add<f32> for Vec4 {
    type Output = Vec4;

    fn add(self, other: f32) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] + other,
                self.x[1] + other,
                self.x[2] + other,
                self.x[3] + other,
            ],
        }
    }
}
// += Vec4
impl ops::AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, other: Vec4) {
        self.x[0] += other.x[0];
        self.x[1] += other.x[1];
        self.x[2] += other.x[2];
        self.x[3] += other.x[3];
    }
}
// += f32
impl ops::AddAssign<f32> for Vec4 {
    fn add_assign(&mut self, other: f32) {
        self.x[0] += other;
        self.x[1] += other;
        self.x[2] += other;
        self.x[3] += other;
    }
}
// - Vec4
impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;
    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] - other.x[0],
                self.x[1] - other.x[1],
                self.x[2] - other.x[2],
                self.x[3] - other.x[3],
            ],
        }
    }
}
// - f32
impl ops::Sub<f32> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: f32) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] - other,
                self.x[1] - other,
                self.x[2] - other,
                self.x[3] - other,
            ],
        }
    }
}
// -= Vec4
impl ops::SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, other: Vec4) {
        self.x[0] -= other.x[0];
        self.x[1] -= other.x[1];
        self.x[2] -= other.x[2];
        self.x[3] -= other.x[3];
    }
}
// -= f32
impl ops::SubAssign<f32> for Vec4 {
    fn sub_assign(&mut self, other: f32) {
        self.x[0] -= other;
        self.x[1] -= other;
        self.x[2] -= other;
        self.x[3] -= other;
    }
}
// * Vec4
impl ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] * other.x[0],
                self.x[1] * other.x[1],
                self.x[2] * other.x[2],
                self.x[3] * other.x[3],
            ],
        }
    }
}
// * f32
impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] * other,
                self.x[1] * other,
                self.x[2] * other,
                self.x[3] * other,
            ],
        }
    }
}
// *= Vec4
impl ops::MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, other: Vec4) {
        self.x[0] *= other.x[0];
        self.x[1] *= other.x[1];
        self.x[2] *= other.x[2];
        self.x[3] *= other.x[3];
    }
}
// *=
impl ops::MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, other: f32) {
        self.x[0] *= other;
        self.x[1] *= other;
        self.x[2] *= other;
        self.x[3] *= other;
    }
}
// / Vec4
impl ops::Div<Vec4> for Vec4 {
    type Output = Vec4;
    fn div(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] / other.x[0],
                self.x[1] / other.x[1],
                self.x[2] / other.x[2],
                self.x[3] / other.x[3],
            ],
        }
    }
}
// / f32
impl ops::Div<f32> for Vec4 {
    type Output = Vec4;
    fn div(self, other: f32) -> Vec4 {
        Vec4 {
            x: [
                self.x[0] / other,
                self.x[1] / other,
                self.x[2] / other,
                self.x[3] / other,
            ],
        }
    }
}
// /= Vec4
impl ops::DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, other: Vec4) {
        self.x[0] /= other.x[0];
        self.x[1] /= other.x[1];
        self.x[2] /= other.x[2];
        self.x[3] /= other.x[3];
    }
}
// /=
impl ops::DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, other: f32) {
        if other != 0.0 {
            self.x[0] /= other;
            self.x[1] /= other;
            self.x[2] /= other;
            self.x[3] /= other;
        }
    }
}
impl ops::Index<usize> for Vec4 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.x[index]
    }
}
impl ops::IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.x[index]
    }
}
impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vec4({}, {}, {}, {})",
            self.x[0], self.x[1], self.x[2], self.x[3]
        )
    }
}
