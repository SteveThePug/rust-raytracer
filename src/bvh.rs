use crate::{ray::*, EPSILON};
use nalgebra::{Point3, Vector3};

// BOUNDING BOX -----------------------------------------------------------------
#[derive(Clone)]
pub struct AABB {
    pub bln: Point3<f64>,
    pub trf: Point3<f64>,
}

impl AABB {
    // New box with respective coordinates
    pub fn new(bln: Point3<f64>, trf: Point3<f64>) -> AABB {
        let bln = bln + Vector3::new(EPSILON, EPSILON, EPSILON);
        let trf = trf - Vector3::new(EPSILON, EPSILON, EPSILON);
        AABB { bln, trf }
    }
    // Intersect bounding box exactly
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
            if intersect.x > bln.x
                || intersect.x < trf.x
                || intersect.y > bln.y
                || intersect.y < trf.y
                || intersect.z > bln.z
                || intersect.z < trf.z
            {
                return true; // Intersection is outside the box
            }
        }
        false
    }
    // Intersect way with some epsilon term
    pub fn intersect_bounding_box_aprox(&self, ray: &Ray) -> bool {
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
    // Get the center of this bounding box
    fn get_centroid(&self) -> Point3<f64> {
        self.bln + (self.trf - self.bln) / 2.0
    }
    // Make a new AABB that contains both
    pub fn join(&self, other: &AABB) -> AABB {
        AABB::new(
            Point3::new(
                self.bln.x.min(other.bln.x),
                self.bln.y.min(other.bln.y),
                self.bln.z.min(other.bln.z),
            ),
            Point3::new(
                self.trf.x.max(other.trf.x),
                self.trf.y.max(other.trf.y),
                self.trf.z.max(other.trf.z),
            ),
        )
    }
    //Grow the AABB to contain the cover the point
    pub fn grow(&self, other: &Point3<f64>) -> AABB {
        AABB::new(
            Point3::new(
                self.bln.x.min(other.x),
                self.bln.y.min(other.y),
                self.bln.z.min(other.z),
            ),
            Point3::new(
                self.trf.x.max(other.x),
                self.trf.y.max(other.y),
                self.trf.z.max(other.z),
            ),
        )
    }
    // Size of AABB
    pub fn size(&self) -> Vector3<f64> {
        self.trf - self.bln
    }
    //Surface area of AABB
    pub fn surface_area(&self) -> f64 {
        let size = self.size();
        2.0 * (size.x * size.y + size.x * size.z + size.y * size.z)
    }
    // Volume of the AABB
    pub fn volume(&self) -> f64 {
        let size = self.size();
        size.x * size.y * size.z
    }
}

pub enum BVHNode<'a> {
    Leaf {
        parent: &'a BVHNode<'a>,
        bounding_box: AABB,
        depth: u32,
    },
    Node {
        parent: Option<&'a BVHNode<'a>>,
        child_l: &'a BVHNode<'a>,
        child_r: &'a BVHNode<'a>,
        depth: u32,
    },
}

impl<'a> BVHNode<'a> {}
