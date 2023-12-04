#[allow(dead_code)]
use nalgebra::Vector3;
// MATERIAL -----------------------------------------------------------------
#[derive(Clone)]
pub struct Material {
    pub kd: Vector3<f32>,
    pub ks: Vector3<f32>,
    pub kr: Vector3<f32>,
    pub shininess: f32,
}

impl Material {
    pub fn new(kd: Vector3<f64>, ks: Vector3<f64>, kr: Vector3<f64>, shininess: f64) -> Material {
        let kd = kd.cast();
        let ks = ks.cast();
        let kr = kr.cast();
        let shininess = shininess as f32;
        Material {
            kd,
            ks,
            kr,
            shininess,
        }
    }
    pub fn magenta() -> Material {
        let kd = Vector3::new(1.0, 0.0, 1.0);
        let ks = Vector3::new(1.0, 0.0, 1.0);
        let kr = Vector3::new(0.0, 0.0, 0.0);
        let shininess = 0.5;
        Material {
            kd,
            ks,
            kr,
            shininess,
        }
    }
    pub fn turquoise() -> Material {
        let kd = Vector3::new(0.25, 0.3, 0.7);
        let ks = Vector3::new(0.25, 0.3, 0.7);
        let kr = Vector3::new(0.0, 0.0, 0.0);
        let shininess = 0.5;
        Material {
            kd,
            ks,
            kr,
            shininess,
        }
    }
    pub fn red() -> Material {
        let kd = Vector3::new(0.8, 0.0, 0.3);
        let ks = Vector3::new(0.8, 0.3, 0.0);
        let kr = Vector3::new(0.0, 0.0, 0.0);
        let shininess = 0.5;
        Material {
            kd,
            ks,
            kr,
            shininess,
        }
    }
    pub fn blue() -> Material {
        let kd = Vector3::new(0.0, 0.3, 0.6);
        let ks = Vector3::new(0.3, 0.0, 0.6);
        let kr = Vector3::new(0.0, 0.0, 0.0);
        let shininess = 0.5;
        Material {
            kd,
            ks,
            kr,
            shininess,
        }
    }
    pub fn green() -> Material {
        let kd = Vector3::new(0.0, 1.0, 0.0);
        let ks = Vector3::new(0.0, 1.0, 0.0);
        let kr = Vector3::new(0.0, 0.0, 0.0);
        let shininess = 0.5;
        Material {
            kd,
            ks,
            kr,
            shininess,
        }
    }
}
