#[allow(dead_code)]
use crate::ray::Ray;
use crate::{EPSILON, EPSILON_VECTOR, INFINITY};
use nalgebra::{distance, Point3, Unit, Vector3};
use roots::{find_roots_cubic, find_roots_quadratic, find_roots_quartic, Roots};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

// MATERIAL -----------------------------------------------------------------
#[derive(Clone)]
pub struct Material {
    pub kd: Vector3<f64>,
    pub ks: Vector3<f64>,
    pub shininess: f64,
}
impl Material {
    pub fn new(kd: Vector3<f64>, ks: Vector3<f64>, shininess: f64) -> Arc<Self> {
        Arc::new(Material { kd, ks, shininess })
    }
    pub fn magenta() -> Arc<Self> {
        let kd = Vector3::new(1.0, 0.0, 1.0);
        let ks = Vector3::new(1.0, 0.0, 1.0);
        let shininess = 0.5;
        Arc::new(Material { kd, ks, shininess })
    }
    pub fn turquoise() -> Arc<Self> {
        let kd = Vector3::new(0.25, 0.3, 0.7);
        let ks = Vector3::new(0.25, 0.3, 0.7);
        let shininess = 0.5;
        Arc::new(Material { kd, ks, shininess })
    }
    pub fn red() -> Arc<Self> {
        let kd = Vector3::new(0.8, 0.0, 0.3);
        let ks = Vector3::new(0.8, 0.3, 0.0);
        let shininess = 0.5;
        Arc::new(Material { kd, ks, shininess })
    }
    pub fn blue() -> Arc<Self> {
        let kd = Vector3::new(0.0, 0.3, 0.6);
        let ks = Vector3::new(0.3, 0.0, 0.6);
        let shininess = 0.5;
        Arc::new(Material { kd, ks, shininess })
    }
    pub fn green() -> Arc<Self> {
        let kd = Vector3::new(0.0, 1.0, 0.0);
        let ks = Vector3::new(0.0, 1.0, 0.0);
        let shininess = 0.5;
        Arc::new(Material { kd, ks, shininess })
    }
}
// INTERSECTION -----------------------------------------------------------------
pub struct Intersection {
    // Information about an intersection
    pub point: Point3<f64>,
    pub normal: Unit<Vector3<f64>>,
    pub incidence: Unit<Vector3<f64>>,
    pub material: Arc<Material>,
    pub distance: f64,
}
// BOUNDING BOX -----------------------------------------------------------------
#[derive(Clone)]
struct BoundingBox {
    bln: Point3<f64>,
    trf: Point3<f64>,
}

impl BoundingBox {
    fn new(bln: Point3<f64>, trf: Point3<f64>) -> Self {
        let bln = bln - EPSILON_VECTOR;
        let trf = trf + EPSILON_VECTOR;
        BoundingBox { bln, trf }
    }
    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        let t1 = (self.bln - ray.a).component_div(&ray.b);
        let t2 = (self.trf - ray.a).component_div(&ray.b);

        let tmin = t1.inf(&t2).min();
        let tmax = t1.sup(&t2).max();

        if tmax >= tmin {
            Some(ray.at_t(tmin))
        } else {
            None
        }
    }
    fn get_centroid(&self) -> Point3<f64> {
        self.bln + (self.trf - self.bln) / 2.0
    }
}
// PRIMITIVE TRAIT -----------------------------------------------------------------
pub trait Primitive: Send + Sync {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>>;
    fn get_material(&self) -> Arc<Material>;
}

// SPHERE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Sphere {
    position: Point3<f64>,
    radius: f64,
    bounding_box: BoundingBox,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new(position: Point3<f64>, radius: f64, material: Arc<Material>) -> Arc<dyn Primitive> {
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        let bounding_box = BoundingBox::new(bln, trf);
        Arc::new(Sphere {
            position,
            radius,
            bounding_box,
            material,
        })
    }

    pub fn unit(material: Arc<Material>) -> Arc<dyn Primitive> {
        Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, material)
    }
}

impl Primitive for Sphere {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let pos = &ray.a;
        let dir = &ray.b;

        let l = pos - self.position;

        let a = dir.dot(dir);
        let b = 2.0 * l.dot(dir);
        let c = l.dot(&l) - self.radius * self.radius;

        let t = match find_roots_quadratic(a, b, c) {
            Roots::No(_) => return None,
            Roots::One([x1]) => x1,
            Roots::Two([x1, x2]) => {
                if x1 <= 0.0 && x2 <= 0.0 {
                    return None;
                } else {
                    if x1.abs() < x2.abs() {
                        x1
                    } else {
                        x2
                    }
                }
            }
            _ => return None,
        };

        let intersect = ray.at_t(t);
        let normal = Unit::new_normalize(intersect - self.position);
        Some(Intersection {
            point: intersect,
            normal,
            incidence: ray.b,
            material: Arc::clone(&self.material),
            distance: t,
        })
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        return self.bounding_box.intersect_bounding_box(ray);
    }
}

// CIRCLE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Circle {
    position: Point3<f64>,
    radius: f64,
    normal: Vector3<f64>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Circle {
    pub fn new(
        position: Point3<f64>,
        radius: f64,
        normal: Vector3<f64>,
        material: Arc<Material>,
    ) -> Arc<dyn Primitive> {
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        let bounding_box = BoundingBox::new(bln, trf);
        Arc::new(Circle {
            position,
            radius,
            normal: normal.normalize(),
            material,
            bounding_box,
        })
    }

    pub fn unit(material: Arc<Material>) -> Arc<dyn Primitive> {
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let radius = 1.0;
        let material = material;

        let bln = Point3::new(-radius, 0.0, -EPSILON);
        let trf = Point3::new(radius, 0.0, EPSILON);
        let bounding_box = BoundingBox { bln, trf };

        Arc::new(Circle {
            position,
            normal,
            radius,
            material,
            bounding_box,
        })
    }
}

impl Primitive for Circle {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let constant = self.position.coords.dot(&self.normal);
        let denominator = ray.b.dot(&self.normal);
        let t = (constant - ray.a.coords.dot(&self.normal)) / denominator;
        if t > INFINITY {
            return None;
        };
        let intersect = ray.at_t(t);
        let distance = distance(&intersect, &self.position);
        match distance >= self.radius {
            true => return None,
            false => {
                return Some(Intersection {
                    point: intersect,
                    normal: Unit::new_normalize(self.normal),
                    incidence: ray.b,
                    material: Arc::clone(&self.material),
                    distance: t,
                })
            }
        }
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// CYLINDER -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cylinder {
    radius: f64,
    base: f64,
    top: f64,
    base_circle: Arc<dyn Primitive>,
    top_circle: Arc<dyn Primitive>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Cylinder {
    pub fn new(radius: f64, base: f64, top: f64, material: Arc<Material>) -> Arc<dyn Primitive> {
        let base_circle = Circle::new(
            Point3::new(0.0, base, 0.0),
            radius,
            Vector3::new(0.0, -1.0, 0.0),
            Arc::clone(&material),
        );
        let top_circle = Circle::new(
            Point3::new(0.0, top, 0.0),
            radius,
            Vector3::new(0.0, 1.0, 0.0),
            Arc::clone(&material),
        );
        let bln = Point3::new(-radius, base, -radius);
        let trf = Point3::new(radius, top, radius);
        Arc::new(Cylinder {
            radius,
            base,
            top,
            base_circle,
            top_circle,
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
}

impl Primitive for Cylinder {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let point = &ray.a;
        let dir = &ray.b;
        let (ax, _ay, az) = (point.x, point.y, point.z);
        let (bx, _by, bz) = (dir.x, dir.y, dir.z);
        let a = bx * bx + bz * bz;
        let b = 2.0 * ax * bx + 2.0 * az * bz;
        let c = ax * ax + az * az - self.radius * self.radius;

        let t = match find_roots_quadratic(a, b, c) {
            Roots::No(_) => return None,
            Roots::One([x1]) => Some(x1),
            Roots::Two([x1, x2]) => {
                if x1 <= 0.0 && x2 <= 0.0 {
                    return None;
                } else {
                    if x1.abs() < x2.abs() {
                        Some(x1)
                    } else {
                        Some(x2)
                    }
                }
            }
            _ => return None,
        };

        let cylinder_intersect = match t {
            None => None,
            Some(t) => {
                let intersect = ray.at_t(t);
                if intersect.y >= self.base && intersect.y <= self.top {
                    let normal = Vector3::new(2.0 * intersect.x, 0.0, 2.0 * intersect.z);
                    Some(Intersection {
                        point: intersect,
                        normal: Unit::new_normalize(normal),
                        material: Arc::clone(&self.material),
                        incidence: ray.b,
                        distance: t,
                    })
                } else {
                    None
                }
            }
        };
        let base_intersect = self.base_circle.intersect_ray(ray);
        let top_intersect = self.top_circle.intersect_ray(ray);
        match (cylinder_intersect, base_intersect, top_intersect) {
            (None, None, None) => None,
            (Some(intersect), None, None) => Some(intersect),
            (None, Some(intersect), None) => Some(intersect),
            (None, None, Some(intersect)) => Some(intersect),
            (Some(intersect), Some(intersect_base), None) => {
                let cylinder_distance = distance(&ray.a, &intersect.point);
                let base_distance = distance(&ray.a, &intersect_base.point);
                match cylinder_distance < base_distance {
                    true => Some(intersect),
                    false => Some(intersect_base),
                }
            }
            (Some(intersect), None, Some(intersect_top)) => {
                let cylinder_distance = distance(&ray.a, &intersect.point);
                let top_distance = distance(&ray.a, &intersect_top.point);
                match cylinder_distance < top_distance {
                    true => Some(intersect),
                    false => Some(intersect_top),
                }
            }
            (None, Some(intersect_base), Some(intersect_top)) => {
                let base_distance = distance(&ray.a, &intersect_base.point);
                let top_distance = distance(&ray.a, &intersect_top.point);
                match base_distance < top_distance {
                    true => Some(intersect_base),
                    false => Some(intersect_top),
                }
            }
            _ => None,
        }
    }
    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// CONE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cone {
    radius: f64,
    base: f64,
    apex: f64,
    circle: Arc<dyn Primitive>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Cone {
    pub fn new(radius: f64, apex: f64, base: f64, material: Arc<Material>) -> Arc<dyn Primitive> {
        let circle = Circle::new(
            Point3::new(0.0, base, 0.0),
            radius,
            Vector3::new(0.0, 1.0, 0.0),
            Arc::clone(&material),
        );
        let bln = Point3::new(-radius, base, -radius);
        let trf = Point3::new(radius, base + apex, radius);
        Arc::new(Cone {
            radius: radius / 2.0,
            base,
            apex,
            circle,
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
    pub fn unit(material: Arc<Material>) -> Arc<dyn Primitive> {
        Cone::new(1.0, 2.0, -1.0, material)
    }

    pub fn get_normal(&self, intersect: Point3<f64>) -> Vector3<f64> {
        let r = self.radius;
        let h = self.apex;
        let (x, y, z) = (intersect.x, intersect.y, intersect.z);
        let normal = Vector3::new(2.0 * x, 2.0 * r * r * (h - y), 2.0 * z).normalize();
        normal
    }
}

impl Primitive for Cone {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let point = &ray.a;
        let dir = &ray.b;
        let (r, h) = (self.radius, self.apex);
        let (a1, a2, a3) = (point.x, point.y, point.z);
        let (b1, b2, b3) = (dir.x, dir.y, dir.z);
        let r2 = r * r;
        let a = b1 * b1 + b3 * b3 - r2 * (b2 * b2);
        let b = 2.0 * (a1 * b1 + a3 * b3 + r2 * (h * b2 - a2 * b2));
        let c = a1 * a1 + a3 * a3 + r2 * (2.0 * h * a2 - h * h - a2 * a2);

        let t = match find_roots_quadratic(a, b, c) {
            Roots::No(_) => None,
            Roots::One([x1]) => Some(x1),
            Roots::Two([x1, x2]) => {
                if x1 <= 0.0 && x2 <= 0.0 {
                    None
                } else {
                    if x1.abs() < x2.abs() {
                        Some(x1)
                    } else {
                        Some(x2)
                    }
                }
            }
            _ => None,
        };

        let cone_intersect = match t {
            None => None,
            Some(t) => {
                let intersect = ray.at_t(t);
                match intersect.y >= self.base && intersect.y <= self.apex {
                    true => Some(Intersection {
                        point: intersect,
                        normal: Unit::new_normalize(self.get_normal(intersect)),
                        material: Arc::clone(&self.material),
                        incidence: ray.b,
                        distance: t,
                    }),
                    false => None,
                }
            }
        };

        let circle_intersect = self.circle.intersect_ray(ray);

        match (cone_intersect, circle_intersect) {
            (None, None) => None,
            (Some(cone_intersect), None) => Some(cone_intersect),
            (None, Some(circle_intersect)) => Some(circle_intersect),
            (Some(cone_intersect), Some(circle_intersect)) => {
                let circle_distance = distance(&ray.a, &circle_intersect.point);
                let cone_distance = distance(&ray.a, &cone_intersect.point);
                match cone_distance < circle_distance {
                    true => Some(cone_intersect),
                    false => Some(circle_intersect),
                }
            }
        }
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// RECTANGLE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Rectangle {
    position: Point3<f64>,
    normal: Vector3<f64>,
    width_direction: Vector3<f64>,
    material: Arc<Material>,
    width: f64,
    height: f64,
    bounding_box: BoundingBox,
}

impl Rectangle {
    pub fn new(
        position: Point3<f64>,
        normal: Vector3<f64>,
        width_direction: Vector3<f64>,
        width: f64,
        height: f64,
        material: Arc<Material>,
    ) -> Arc<dyn Primitive> {
        let normal = normal.normalize();
        let width_direction = width_direction.normalize();
        let height_direction = width_direction.cross(&normal);
        let bln = position - width / 2.0 * width_direction - height / 2.0 * height_direction;
        let trf = position + width / 2.0 * width_direction + height / 2.0 * height_direction;
        Arc::new(Rectangle {
            position,
            normal: normal.normalize(),
            width_direction: width_direction.normalize(),
            width,
            height,
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
    pub fn unit(material: Arc<Material>) -> Arc<dyn Primitive> {
        Rectangle::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            2.0,
            2.0,
            material,
        )
    }
}

impl Primitive for Rectangle {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let constant = self.position.coords.dot(&self.normal);
        let denominator = ray.b.dot(&self.normal);
        let t = (constant - ray.a.coords.dot(&self.normal)) / denominator;

        if t > INFINITY {
            return None;
        }

        let intersect = ray.at_t(t);
        let height_direction = self.width_direction.cross(&self.normal);
        let (w2, h2) = (self.width / 2.0, self.height / 2.0);
        let r1 = w2 * self.width_direction;
        let r2 = h2 * height_direction;
        let pi = intersect - self.position;
        let pi_dot_r1 = pi.dot(&r1);
        let pi_dot_r2 = pi.dot(&r2);

        if pi_dot_r1 >= -w2 && pi_dot_r1 <= w2 && pi_dot_r2 >= -h2 && pi_dot_r2 <= h2 {
            return Some(Intersection {
                point: intersect,
                normal: Unit::new_normalize(self.normal),
                incidence: ray.b,
                material: Arc::clone(&self.material),
                distance: t,
            });
        }
        None
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// BOX -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cube {
    width: f64,
    height: f64,
    depth: f64,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Cube {
    pub fn new(width: f64, height: f64, depth: f64, material: Arc<Material>) -> Arc<dyn Primitive> {
        let trf = Point3::new(width / 2.0, height / 2.0, depth / 2.0);
        let bln = Point3::new(-width / 2.0, -height / 2.0, -depth / 2.0);
        Arc::new(Cube {
            width,
            height,
            depth,
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
    pub fn unit(material: Arc<Material>) -> Arc<dyn Primitive> {
        Cube::new(2.0, 2.0, 2.0, material)
    }
}

impl Primitive for Cube {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        // Compute the minimum and maximum t-values for each axis of the bounding box
        let t1 = (self.bounding_box.bln - ray.a).component_div(&ray.b);
        let t2 = (self.bounding_box.trf - ray.a).component_div(&ray.b);

        // Find the largest minimum t-value and the smallest maximum t-value among the axes
        let tmin = t1.inf(&t2).max();
        let tmax = t1.sup(&t2).min();

        // Check if there's an intersection between tmin and tmax
        if tmax >= tmin {
            // The ray intersects the box, and tmin is the entry point, tmax is the exit point
            let intersect = ray.at_t(tmin);

            // Check if the intersection is outside the box
            if intersect.x < -self.width / 2.0
                || intersect.x > self.width / 2.0
                || intersect.y < -self.height / 2.0
                || intersect.y > self.height / 2.0
                || intersect.z < -self.depth / 2.0
                || intersect.z > self.depth / 2.0
            {
                return None; // Intersection is outside the box
            }

            //Get normal of intersection point
            let normal = if tmin == t1.x {
                Vector3::new(-1.0, 0.0, 0.0)
            } else if tmin == t1.y {
                Vector3::new(0.0, -1.0, 0.0)
            } else if tmin == t1.z {
                Vector3::new(0.0, 0.0, -1.0)
            } else if tmin == t2.x {
                Vector3::new(1.0, 0.0, 0.0)
            } else if tmin == t2.y {
                Vector3::new(0.0, 1.0, 0.0)
            } else {
                Vector3::new(0.0, 0.0, 1.0)
            };

            Some(Intersection {
                point: intersect,
                normal: Unit::new_normalize(normal),
                incidence: ray.b,
                material: Arc::clone(&self.material),
                distance: tmin,
            })
        } else {
            None // No intersection with the box
        }
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}

// TRIANGLE -----------------------------------------------------------------
struct Triangle {
    u: Point3<f64>,
    v: Point3<f64>,
    w: Point3<f64>,
    normal: Vector3<f64>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Triangle {
    fn new(
        u: Point3<f64>,
        v: Point3<f64>,
        w: Point3<f64>,
        material: Arc<Material>,
    ) -> Arc<dyn Primitive> {
        let uv = v - u;
        let uw = w - u;
        let normal = uv.cross(&uw).normalize();
        let bln = u.inf(&v).inf(&w);
        let trf = u.sup(&v).sup(&w);
        let bounding_box = BoundingBox { bln, trf };
        Arc::new(Triangle {
            u,
            v,
            w,
            normal,
            material,
            bounding_box,
        })
    }
    pub fn unit(material: Arc<Material>) -> Arc<dyn Primitive> {
        let u = Point3::new(-1.0, 0.0, -1.0);
        let v = Point3::new(0.0, 0.0, 1.0);
        let w = Point3::new(1.0, 0.0, -1.0);
        let material = material;
        Triangle::new(u, v, w, material)
    }
}

impl Primitive for Triangle {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let constant = self.u.coords.dot(&self.normal);
        let denominator = ray.b.dot(&self.normal);
        let t = (constant - ray.a.coords.dot(&self.normal)) / denominator;

        if t > INFINITY {
            return None;
        }

        let intersect = ray.at_t(t);

        let uv = self.v - self.u;
        let vw = self.w - self.v;
        let wu = self.u - self.w;

        let ui = intersect - self.u;
        let vi = intersect - self.v;
        let wi = intersect - self.w;

        let u_cross = uv.cross(&ui);
        let v_cross = vw.cross(&vi);
        let w_cross = wu.cross(&wi);

        let normal = self.normal;

        if u_cross.dot(&normal) >= 0.0 && v_cross.dot(&normal) >= 0.0 && w_cross.dot(&normal) >= 0.0
        {
            Some(Intersection {
                point: intersect,
                normal: Unit::new_normalize(normal),
                incidence: ray.b,
                material: Arc::clone(&self.material),
                distance: t,
            })
        } else {
            None
        }
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// MESH -----------------------------------------------------------------
struct Mesh {
    triangles: Vec<Arc<Triangle>>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Mesh {
    fn new(triangles: Vec<Arc<Triangle>>, material: Arc<Material>) -> Arc<dyn Primitive> {
        // Calculate the bounding box for the entire mesh based on the bounding boxes of individual triangles
        let bounding_box = Mesh::compute_bounding_box(&triangles);

        Arc::new(Mesh {
            triangles,
            material,
            bounding_box,
        })
    }

    fn compute_bounding_box(triangles: &Vec<Arc<Triangle>>) -> BoundingBox {
        let mut bln = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut trf = -bln;
        for triangle in triangles {
            bln = bln.inf(&triangle.u);
            bln = bln.inf(&triangle.v);
            bln = bln.inf(&triangle.w);

            trf = trf.sup(&triangle.u);
            trf = trf.sup(&triangle.v);
            trf = trf.sup(&triangle.w);
        }
        BoundingBox { bln, trf }
    }

    fn from_file(filename: &str, material: Arc<Material>) -> Arc<dyn Primitive> {
        let mut triangles: Vec<Arc<dyn Primitive>> = Vec::new();
        let mut vertices: Vec<Point3<f64>> = Vec::new();

        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let mut parts = line.split_whitespace();
                if let Some(keyword) = parts.next() {
                    match keyword {
                        "v" => {
                            // Parse vertex coordinates
                            if let (Some(x_str), Some(y_str), Some(z_str)) =
                                (parts.next(), parts.next(), parts.next())
                            {
                                let x: f64 = x_str.parse().expect("Failed to parse vertex X");
                                let y: f64 = y_str.parse().expect("Failed to parse vertex Y");
                                let z: f64 = z_str.parse().expect("Failed to parse vertex Z");
                                vertices.push(Point3::new(x, y, z));
                            }
                        }
                        "f" => {
                            // Parse face indices
                            if let (Some(v1_str), Some(v2_str), Some(v3_str)) =
                                (parts.next(), parts.next(), parts.next())
                            {
                                let v1: usize =
                                    v1_str.parse().expect("Failed to parse vertex index");
                                let v2: usize =
                                    v2_str.parse().expect("Failed to parse vertex index");
                                let v3: usize =
                                    v3_str.parse().expect("Failed to parse vertex index");
                                // Indices in OBJ files are 1-based, so subtract 1 to convert to 0-based.
                                let a = vertices[v1 - 1];
                                let b = vertices[v2 - 1];
                                let c = vertices[v3 - 1];
                                triangles.push(Triangle::new(a, b, c, Arc::clone(&material)));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        todo!();
    }
}

impl Primitive for Mesh {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_distance = INFINITY;
        let mut closest_intersect: Option<Intersection> = None;

        for triangle in &self.triangles {
            match triangle.intersect_ray(ray) {
                Some(intersect) => {
                    let distance = intersect.distance;
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_intersect = Some(intersect);
                    };
                }
                None => continue,
            }
        }

        closest_intersect
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

#[derive(Clone)]
pub struct SteinerSurface {
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl SteinerSurface {
    pub fn new(material: Arc<Material>) -> Arc<dyn Primitive> {
        // I need to find the bounding box for this shape
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        Arc::new(SteinerSurface {
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
}

impl Primitive for SteinerSurface {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;

        let t0 = a.powf(2.0) * c.powf(2.0)
            + a.powf(2.0) * e.powf(2.0)
            + c.powf(2.0) * e.powf(2.0)
            + c * e.powf(2.0);
        let t1 = 2.0 * a * b * c.powf(2.0)
            + 2.0 * a.powf(2.0) * c * d
            + 2.0 * a * b * e.powf(2.0)
            + 2.0 * c * d * e.powf(2.0)
            + 2.0 * a.powf(2.0) * e * f
            + 2.0 * c.powf(2.0) * e * f
            + d * e.powf(2.0)
            + 2.0 * c * e * f.powf(2.0);
        let t2 = b.powf(2.0) * c.powf(2.0)
            + 4.0 * a * b * c * d
            + a.powf(2.0) * d.powf(2.0)
            + b.powf(2.0) * e.powf(2.0)
            + d.powf(2.0) * e.powf(2.0)
            + 4.0 * a * b * e * f
            + 4.0 * c * d * e * f
            + a.powf(2.0) * f.powf(2.0)
            + c.powf(2.0) * f.powf(2.0)
            + 2.0 * d * e * f
            + c * f.powf(2.0);
        let t3 = 2.0 * b.powf(2.0) * c * d
            + 2.0 * a * b * d.powf(2.0)
            + 2.0 * b.powf(2.0) * e * f
            + 2.0 * d.powf(2.0) * e * f
            + 2.0 * a * b * f.powf(2.0)
            + 2.0 * c * d * f.powf(2.0)
            + d.powf(2.0) * f.powf(2.0);
        let t4 = b.powf(2.0) * d.powf(2.0) + b.powf(2.0) * f.powf(2.0) + d.powf(2.0) * f.powf(2.0);

        let t = match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        };

        let t = match t {
            Some(t) => t,
            None => return None,
        };

        //Now we have the smallest non-zero t
        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let normal = Unit::new_normalize(Vector3::new(
            2.0 * x * y * y + 2.0 * x * z * z + y * z,
            2.0 * x * x * y + 2.0 * z * z * y + x * z,
            2.0 * x * x * z + 2.0 * z * y * y + x * y,
        ));

        Some(Intersection {
            point,
            normal,
            incidence: ray.b,
            material: Arc::clone(&self.material),
            distance: t,
        })
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}

fn smallest_non_zero(arr: &[f64]) -> Option<f64> {
    for &num in arr {
        if num >= 0.0 {
            return Some(num);
        }
    }
    None
}

#[derive(Clone)]
pub struct Torus {
    inner_rad: f64,
    outer_rad: f64,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}
impl Torus {
    pub fn new(inner_rad: f64, outer_rad: f64, material: Arc<Material>) -> Arc<dyn Primitive> {
        // I need to find the bounding box for this shape
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        Arc::new(Torus {
            inner_rad,
            outer_rad,
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
}
impl Primitive for Torus {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;
        let r1 = self.inner_rad;
        let r2 = self.outer_rad;
        let t0 = r2.powf(2.0).powf(2.0) - 2.0 * r2.powf(2.0) * a.powf(2.0) + a.powf(2.0).powf(2.0)
            - 2.0 * r2.powf(2.0) * c.powf(2.0)
            + 2.0 * a.powf(2.0) * c.powf(2.0)
            + c.powf(2.0).powf(2.0)
            + 2.0 * r2.powf(2.0) * e.powf(2.0)
            + 2.0 * a.powf(2.0) * e.powf(2.0)
            + 2.0 * c.powf(2.0) * e.powf(2.0)
            + e.powf(2.0).powf(2.0)
            - 2.0 * r2.powf(2.0) * r1.powf(2.0)
            - 2.0 * a.powf(2.0) * r1.powf(2.0)
            - 2.0 * c.powf(2.0) * r1.powf(2.0)
            - 2.0 * e.powf(2.0) * r1.powf(2.0)
            + r1.powf(2.0).powf(2.0);
        let t1 = -4.0 * r2.powf(2.0) * a * b + 4.0 * a.powf(3.0) * b + 4.0 * a * b * c.powf(2.0)
            - 4.0 * r2.powf(2.0) * c * d
            + 4.0 * a.powf(2.0) * c * d
            + 4.0 * c.powf(3.0) * d
            + 4.0 * a * b * e.powf(2.0)
            + 4.0 * c * d * e.powf(2.0)
            + 4.0 * r2.powf(2.0) * e * f
            + 4.0 * a.powf(2.0) * e * f
            + 4.0 * c.powf(2.0) * e * f
            + 4.0 * e.powf(3.0) * f
            - 4.0 * a * b * r1.powf(2.0)
            - 4.0 * c * d * r1.powf(2.0)
            - 4.0 * e * f * r1.powf(2.0);
        let t2 = -2.0 * r2.powf(2.0) * b.powf(2.0)
            + 6.0 * a.powf(2.0) * b.powf(2.0)
            + 2.0 * b.powf(2.0) * c.powf(2.0)
            + 8.0 * a * b * c * d
            - 2.0 * r2.powf(2.0) * d.powf(2.0)
            + 2.0 * a.powf(2.0) * d.powf(2.0)
            + 6.0 * c.powf(2.0) * d.powf(2.0)
            + 2.0 * b.powf(2.0) * e.powf(2.0)
            + 2.0 * d.powf(2.0) * e.powf(2.0)
            + 8.0 * a * b * e * f
            + 8.0 * c * d * e * f
            + 2.0 * r2.powf(2.0) * f.powf(2.0)
            + 2.0 * a.powf(2.0) * f.powf(2.0)
            + 2.0 * c.powf(2.0) * f.powf(2.0)
            + 6.0 * e.powf(2.0) * f.powf(2.0)
            - 2.0 * b.powf(2.0) * r1.powf(2.0)
            - 2.0 * d.powf(2.0) * r1.powf(2.0)
            - 2.0 * f.powf(2.0) * r1.powf(2.0);
        let t3 = 4.0 * a * b.powf(3.0)
            + 4.0 * b.powf(2.0) * c * d
            + 4.0 * a * b * d.powf(2.0)
            + 4.0 * c * d.powf(3.0)
            + 4.0 * b.powf(2.0) * e * f
            + 4.0 * d.powf(2.0) * e * f
            + 4.0 * a * b * f.powf(2.0)
            + 4.0 * c * d * f.powf(2.0)
            + 4.0 * e * f.powf(3.0);
        let t4 = b.powf(4.0)
            + 2.0 * b.powf(2.0) * d.powf(2.0)
            + d.powf(4.0)
            + 2.0 * b.powf(2.0) * f.powf(2.0)
            + 2.0 * d.powf(2.0) * f.powf(2.0)
            + f.powf(4.0);

        let t = match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        };

        let t = match t {
            Some(t) => t,
            None => return None,
        };

        //Now we have the smallest non-zero t
        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = 4.0 * (r2.powf(2.0) - r1.powf(2.0) + x.powf(2.0) + y.powf(2.0) + z.powf(2.0)) * r2
            - 8.0 * (x.powf(2.0) + y.powf(2.0)) * r2;
        let dy =
            -4.0 * (r2.powf(2.0) - r1.powf(2.0) + x.powf(2.0) + y.powf(2.0) + z.powf(2.0)) * r1;
        let dz = -8.0 * r2.powf(2.0) * x
            + 4.0 * (r2.powf(2.0) - r1.powf(2.0) + x.powf(2.0) + y.powf(2.0) + z.powf(2.0)) * x;
        let normal = Unit::new_normalize(Vector3::new(dx, dy, dz));

        Some(Intersection {
            point,
            normal,
            incidence: ray.b,
            material: Arc::clone(&self.material),
            distance: t,
        })
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}

#[derive(Clone)]
pub struct AdamShape {
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl AdamShape {
    pub fn new(material: Arc<Material>) -> Arc<dyn Primitive> {
        // I need to find the bounding box for this shape
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        Arc::new(AdamShape {
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
}

impl Primitive for AdamShape {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;

        let t0 = a.powf(3.0) + a * c * e + a * c;
        let t1 = 3.0 * a.powf(2.0) * b + b * c * e + a * d * e + a * c * f + b * c + a * d;
        let t2 = 3.0 * a * b.powf(2.0) + b * d * e + b * c * f + a * d * f + b * d;
        let t3 = b.powf(3.0) + b * d * f;

        let t = match find_roots_cubic(t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        };

        let t = match t {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = 3.0 * x.powf(2.0) + y * z + y;
        let dy = x * z + x;
        let dz = x * y;
        let normal = Unit::new_normalize(Vector3::new(dx, dy, dz));

        Some(Intersection {
            point,
            normal,
            incidence: ray.b,
            material: Arc::clone(&self.material),
            distance: t,
        })
    }
    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}
#[derive(Clone)]
pub struct AdamShape2 {
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl AdamShape2 {
    pub fn new(material: Arc<Material>) -> Arc<dyn Primitive> {
        // I need to find the bounding box for this shape
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        Arc::new(AdamShape2 {
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
}

impl Primitive for AdamShape2 {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;

        let t0 = a.powf(2.0) * c + a * c * e + a * c + c * e;
        let t1 = 2.0 * a * b * c
            + a.powf(2.0) * d
            + b * c * e
            + a * d * e
            + a * c * f
            + b * c
            + a * d
            + d * e
            + c * f;
        let t2 =
            b.powf(2.0) * c + 2.0 * a * b * d + b * d * e + b * c * f + a * d * f + b * d + d * f;
        let t3 = b.powf(2.0) * d + b * d * f;

        let t = match find_roots_cubic(t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        };

        let t = match t {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = 2.0 * x * y + y * z + y.powf(2.0);
        let dy = x.powf(2.0) + x * z + x + z;
        let dz = x * y + y;
        let normal = Unit::new_normalize(Vector3::new(dx, dy, dz));

        Some(Intersection {
            point,
            normal,
            incidence: ray.b,
            material: Arc::clone(&self.material),
            distance: t,
        })
    }
    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}

#[derive(Clone)]
pub struct AdamShape3 {
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl AdamShape3 {
    pub fn new(material: Arc<Material>) -> Arc<dyn Primitive> {
        // I need to find the bounding box for this shape
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        Arc::new(AdamShape3 {
            material,
            bounding_box: BoundingBox { bln, trf },
        })
    }
}

impl Primitive for AdamShape3 {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;

        let t0 = a * c * e.powf(2.0) + a * c * e + a * e + c * e;
        let t1 = b * c * e.powf(2.0)
            + a * d * e.powf(2.0)
            + 2.0 * a * c * e * f
            + b * c * e
            + a * d * e
            + a * c * f
            + b * e
            + d * e
            + a * f
            + c * f;
        let t2 = b * d * e.powf(2.0)
            + 2.0 * b * c * e * f
            + 2.0 * a * d * e * f
            + a * c * f.powf(2.0)
            + b * d * e
            + b * c * f
            + a * d * f
            + b * f
            + d * f;
        let t3 = 2.0 * b * d * e * f + b * c * f.powf(2.0) + a * d * f.powf(2.0) + b * d * f;
        let t4 = b * d * f.powf(2.0);

        let t = match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        };

        let t = match t {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = y * z.powf(2.0) + y * z + z;
        let dy = x * z.powf(2.0) + x * z + z;
        let dz = 2.0 * x * y * z + x * y + x + y;
        let normal = Unit::new_normalize(Vector3::new(dx, dy, dz));

        Some(Intersection {
            point,
            normal,
            incidence: ray.b,
            material: Arc::clone(&self.material),
            distance: t,
        })
    }
    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.bounding_box.intersect_bounding_box(ray)
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}
