use crate::{ray::*, EPSILON};
use nalgebra::{Point3, Vector3};

// BOUNDING BOX -----------------------------------------------------------------
#[derive(Clone)]
pub struct BoundingBox {
    pub bln: Point3<f64>,
    pub trf: Point3<f64>,
}

impl BoundingBox {
    pub fn new(bln: Point3<f64>, trf: Point3<f64>) -> Self {
        let bln = bln + Vector3::new(EPSILON, EPSILON, EPSILON);
        let trf = trf - Vector3::new(EPSILON, EPSILON, EPSILON);
        BoundingBox { bln, trf }
    }
    pub fn intersect_bounding_box(&self, ray: &Ray) -> bool {
        let bln = &self.bln;
        let trf = &self.trf;
        let t1 = (bln - ray.a).component_div(&ray.b);
        let t2 = (trf - ray.a).component_div(&ray.b);

        let tmin = t1.inf(&t2).min();
        let tmax = t1.sup(&t2).max();

        if tmax >= tmin {
            let intersect = ray.at_t(tmin);

            // Check if the intersection is inside the box
            if intersect.x > bln.x - EPSILON
                || intersect.x < trf.x + EPSILON
                || intersect.y > bln.y - EPSILON
                || intersect.y < trf.y + EPSILON
                || intersect.z > bln.z - EPSILON
                || intersect.z < trf.z + EPSILON
            {
                return true; // Intersection is outside the box
            }
        }
        false
    }
    #[allow(dead_code)]
    fn get_centroid(&self) -> Point3<f64> {
        self.bln + (self.trf - self.bln) / 2.0
    }
}
