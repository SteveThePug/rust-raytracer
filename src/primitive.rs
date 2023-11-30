use crate::{
    bvh::AABB,
    ray::{Intersection, Ray},
    {EPSILON, INFINITY},
};

#[allow(dead_code)]
use nalgebra::{distance, Point3, Vector3};
use roots::{find_roots_quadratic, find_roots_quartic, Roots};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
// PRIMITIVE TRAIT -----------------------------------------------------------------
pub trait Primitive {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
    fn get_aabb(&self) -> AABB;
}

// SPHERE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Sphere {
    position: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(position: Point3<f64>, radius: f64) -> Rc<dyn Primitive> {
        Rc::new(Sphere { position, radius })
    }

    pub fn unit() -> Rc<dyn Primitive> {
        Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0)
    }
}

impl Primitive for Sphere {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let pos = ray.a;
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
        let normal = (intersect - self.position).normalize();
        Some(Intersection {
            point: intersect,
            normal,
            distance: t,
        })
    }

    fn get_aabb(&self) -> AABB {
        let radius = self.radius;
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = self.position - radius_vec;
        let trf = self.position + radius_vec;
        AABB::new(bln, trf)
    }
}

// CIRCLE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Circle {
    position: Point3<f64>,
    radius: f64,
    normal: Vector3<f64>,
    constant: f64,
}

impl Circle {
    pub fn new(position: Point3<f64>, radius: f64, normal: Vector3<f64>) -> Rc<dyn Primitive> {
        let normal = normal.normalize();
        let constant = normal.dot(&position.coords);
        Rc::new(Circle {
            position,
            radius,
            normal,
            constant,
        })
    }

    pub fn unit() -> Rc<dyn Primitive> {
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 0.0, -1.0);
        let radius = 1.0;
        Circle::new(position, radius, normal)
    }
}

impl Primitive for Circle {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let n_dot_a = ray.a.coords.dot(&self.normal);
        let n_dot_b = ray.b.dot(&self.normal);
        let t = (self.constant - n_dot_a) / n_dot_b;

        if t > INFINITY {
            return None;
        };

        let intersect = ray.at_t(t);
        //Distance to center of circle
        let distance = distance(&intersect, &self.position).abs();
        match distance <= self.radius {
            true => {
                return Some(Intersection {
                    point: intersect,
                    normal: self.normal,
                    distance: t,
                })
            }
            false => return None,
        }
    }

    fn get_aabb(&self) -> AABB {
        let radius = self.radius;
        let position = self.position;
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        AABB::new(bln, trf)
    }
}

// CYLINDER -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cylinder {
    radius: f64,
    height: f64,
    base_circle: Rc<dyn Primitive>,
    top_circle: Rc<dyn Primitive>,
}

impl Cylinder {
    pub fn new(radius: f64, height: f64) -> Rc<dyn Primitive> {
        let base_circle = Circle::new(
            Point3::new(0.0, 0.0, 0.0),
            radius,
            Vector3::new(0.0, -1.0, 0.0),
        );
        let top_circle = Circle::new(
            Point3::new(0.0, height, 0.0),
            radius,
            Vector3::new(0.0, 1.0, 0.0),
        );
        Rc::new(Cylinder {
            radius,
            height,
            base_circle,
            top_circle,
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
                if intersect.y >= 0.0 && intersect.y <= self.height {
                    let normal = Vector3::new(2.0 * intersect.x, 0.0, 2.0 * intersect.z);
                    Some(Intersection {
                        point: intersect,
                        normal: normal,
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

    fn get_aabb(&self) -> AABB {
        let radius = self.radius;
        let height = self.height;
        let bln = Point3::new(-radius, 0.0, -radius);
        let trf = Point3::new(radius, height, radius);
        AABB::new(bln, trf)
    }
}

// CONE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cone {
    height: f64,
    constant: f64,
    circle: Rc<dyn Primitive>,
}

impl Cone {
    pub fn new(radius: f64, height: f64) -> Rc<dyn Primitive> {
        let circle = Circle::new(
            Point3::new(0.0, 0.0, 0.0),
            radius,
            Vector3::new(0.0, -1.0, 0.0),
        );
        let constant = radius * radius / (height * height);
        Rc::new(Cone {
            height,
            constant,
            circle,
        })
    }
    pub fn unit() -> Rc<dyn Primitive> {
        Cone::new(0.5, 1.0)
    }

    pub fn get_normal(&self, intersect: Point3<f64>) -> Vector3<f64> {
        let (x, y, z) = (intersect.x, intersect.y, intersect.z);
        let dx = 2.0 * x;
        let dy = 2.0 * self.constant * (self.height - y);
        let dz = 2.0 * z;
        Vector3::new(dx, dy, dz).normalize()
    }
}

impl Primitive for Cone {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let k1 = self.constant;
        let k2 = self.height;
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;

        let t0 =
            -c.powf(2.0) * k1 + 2.0 * c * k1 * k2 - k1 * k2.powf(2.0) + a.powf(2.0) + e.powf(2.0);
        let t1 = -2.0 * c * d * k1 + 2.0 * d * k1 * k2 + 2.0 * a * b + 2.0 * e * f;
        let t2 = -d.powf(2.0) * k1 + b.powf(2.0) + f.powf(2.0);

        let t = match find_roots_quadratic(t2, t1, t0) {
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
                match intersect.y >= 0.0 && intersect.y <= self.height {
                    true => Some(Intersection {
                        point: intersect,
                        normal: self.get_normal(intersect),
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
            (Some(cone_intersect), Some(_)) => Some(cone_intersect),
        }
    }

    fn get_aabb(&self) -> AABB {
        let height = self.height;
        let radius = (self.constant * height * height).sqrt();
        let bln = Point3::new(-radius, 0.0, -radius);
        let trf = Point3::new(radius, height, radius);
        AABB::new(bln, trf)
    }
}

// RECTANGLE -----------------------------------------------------------------
// #[derive(Clone)]
// pub struct Rectangle {
//     position: Point3<f64>,
//     normal: Vector3<f64>,
//     width_direction: Vector3<f64>,
//     width: f64,
//     height: f64,
// }

// impl Rectangle {
//     pub fn new(
//         position: Point3<f64>,
//         normal: Vector3<f64>,
//         width_direction: Vector3<f64>,
//         width: f64,
//         height: f64,
//     ) -> Rc<dyn Primitive> {
//         let normal = normal.normalize();
//         let width_direction = width_direction.normalize();
//         let height_direction = width_direction.cross(&normal);
//         Rc::new(Rectangle {
//             position,
//             normal: normal.normalize(),
//             width_direction: width_direction.normalize(),
//             width,
//             height,
//         })
//     }
//     pub fn unit() -> Rc<dyn Primitive> {
//         Rectangle::new(
//             Point3::new(0.0, 0.0, 0.0),
//             Vector3::new(0.0, 1.0, 0.0),
//             Vector3::new(1.0, 0.0, 0.0),
//             2.0,
//             2.0,
//         )
//     }
// }

// impl Primitive for Rectangle {
//     fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
//         let constant = self.position.coords.dot(&self.normal);
//         let denominator = ray.b.dot(&self.normal);
//         let t = (constant - ray.a.coords.dot(&self.normal)) / denominator;

//         if t > INFINITY {
//             return None;
//         }

//         let intersect = ray.at_t(t);
//         let height_direction = self.width_direction.cross(&self.normal);
//         let (w2, h2) = (self.width / 2.0, self.height / 2.0);
//         let r1 = w2 * self.width_direction;
//         let r2 = h2 * height_direction;
//         let pi = intersect - self.position;
//         let pi_dot_r1 = pi.dot(&r1);
//         let pi_dot_r2 = pi.dot(&r2);

//         if pi_dot_r1 >= -w2 && pi_dot_r1 <= w2 && pi_dot_r2 >= -h2 && pi_dot_r2 <= h2 {
//             return Some(Intersection {
//                 point: intersect,
//                 normal: self.normal,
//                 distance: t,
//             });
//         }
//         None
//     }

//     fn get_bounding_box(&self) -> AABB {
//         let position = self.position;
//         let width = self.width;
//         let height = self.height;
//         let width_direction = self.width_direction;
//         let bln = position - width / 2.0 * width_direction - height / 2.0 * height_direction;
//         let trf = position + width / 2.0 * width_direction + height / 2.0 * height_direction;
//         AABB::new(bln, trf);
//         todo!()
//     }
// }

// Cube -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cube {
    bln: Point3<f64>,
    trf: Point3<f64>,
}

impl Cube {
    pub fn new(bln: Point3<f64>, trf: Point3<f64>) -> Rc<dyn Primitive> {
        Rc::new(Cube { bln, trf })
    }

    pub fn unit() -> Rc<dyn Primitive> {
        let bln = Point3::new(-1.0, -1.0, -1.0);
        let trf = Point3::new(1.0, 1.0, 1.0);
        Cube::new(bln, trf)
    }
}

impl Primitive for Cube {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        // Compute the minimum and maximum t-values for each axis of the bounding box
        let bln = self.bln;
        let trf = self.trf;
        let t1 = (bln - ray.a).component_div(&ray.b);
        let t2 = (trf - ray.a).component_div(&ray.b);

        // Find the largest minimum t-value and the smallest maximum t-value among the axes
        let tmin = t1.inf(&t2).max();
        let tmax = t1.sup(&t2).min();

        // Check if there's an intersection between tmin and tmax
        if tmax >= tmin && tmin > EPSILON {
            // The ray intersects the box, and tmin is the entry point, tmax is the exit point
            let intersect = ray.at_t(tmin);

            // Check if the intersection is outside the box
            if intersect.x < bln.x
                || intersect.x > trf.x
                || intersect.y < bln.y
                || intersect.y > trf.y
                || intersect.z < bln.z
                || intersect.z > trf.z
            {
                return None; // Intersection is outside the box
            }

            //Get normal of intersection point
            //t1 is bln t2 is trf
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
                normal: normal,
                distance: tmin,
            })
        } else {
            None // No intersection with the box
        }
    }

    fn get_aabb(&self) -> AABB {
        AABB::new(self.bln, self.trf)
    }
}

// TRIANGLE -----------------------------------------------------------------
// Points u,v,w will be pointing outward from uw x uv
// Using the right hand rule
#[derive(Clone)]
#[allow(dead_code)]
pub struct Triangle {
    u: Point3<f64>,
    v: Point3<f64>,
    w: Point3<f64>,
    normal: Vector3<f64>,
}

impl Triangle {
    pub fn new(u: Point3<f64>, v: Point3<f64>, w: Point3<f64>) -> Rc<dyn Primitive> {
        let uv = v - u;
        let uw = w - u;
        let normal = uw.cross(&uv).normalize();
        Rc::new(Triangle { u, v, w, normal })
    }
    #[allow(dead_code)]
    pub fn unit() -> Rc<dyn Primitive> {
        let u = Point3::new(-1.0, -1.0, 0.0);
        let v = Point3::new(0.0, 1.0, 0.0);
        let w = Point3::new(1.0, -1.0, 0.0);
        Triangle::new(u, v, w)
    }
}

impl Primitive for Triangle {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let (u, v, w) = (self.u, self.v, self.w);
        let e1 = v - u;
        let e2 = w - u;
        let b_cross_e2 = ray.b.cross(&e2);
        let det = e1.dot(&b_cross_e2);

        if det > -EPSILON && det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.a - u;
        let p = inv_det * s.dot(&b_cross_e2);

        if p < 0.0 || p > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(&e1);
        let v = inv_det * ray.b.dot(&s_cross_e1);

        if v < 0.0 || p + v > 1.0 {
            return None;
        }
        let t = inv_det * e2.dot(&s_cross_e1);

        if t > EPSILON
        // ray intersection
        {
            let intersect = ray.at_t(t);
            return Some(Intersection {
                point: intersect,
                normal: self.normal,
                distance: t,
            });
        }
        None
    }

    fn get_aabb(&self) -> AABB {
        let u = self.u;
        let v = self.v;
        let w = self.w;
        let bln = u.inf(&v).inf(&w);
        let trf = u.sup(&v).sup(&w);
        AABB::new(bln, trf)
    }
}

// MESH -----------------------------------------------------------------
#[derive(Clone)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Rc<dyn Primitive> {
        // Calculate the bounding box for the entire mesh based on the bounding boxes of individual triangles
        let bounding_box = Mesh::compute_bounding_box(&triangles);
        Rc::new(Mesh { triangles })
    }

    fn compute_bounding_box(triangles: &Vec<Triangle>) -> AABB {
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
        AABB::new(bln, trf)
    }

    pub fn from_file(filename: &str) -> Rc<dyn Primitive> {
        let mut triangles: Vec<Triangle> = Vec::new();
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
                                let u = vertices[v1 - 1];
                                let v = vertices[v2 - 1];
                                let w = vertices[v3 - 1];
                                let uv = u - v;
                                let uw = w - v;
                                let normal = uv.cross(&uw).normalize();
                                triangles.push(Triangle { u, v, w, normal });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Mesh::new(triangles)
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

    fn get_aabb(&self) -> AABB {
        Mesh::compute_bounding_box(&self.triangles)
    }
}

// TORUS -----------------------------------------------------------------
#[derive(Clone)]
pub struct Torus {
    inner_rad: f64,
    outer_rad: f64,
}

impl Torus {
    pub fn new(inner_rad: f64, outer_rad: f64) -> Rc<dyn Primitive> {
        // I need to find the bounding box for this shape
        Rc::new(Torus {
            inner_rad,
            outer_rad,
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

        let t0 = r2.powf(4.0) - 2.0 * r2.powf(2.0) * a.powf(2.0) + a.powf(4.0)
            - 2.0 * r2.powf(2.0) * c.powf(2.0)
            + 2.0 * a.powf(2.0) * c.powf(2.0)
            + c.powf(4.0)
            + 2.0 * r2.powf(2.0) * e.powf(2.0)
            + 2.0 * a.powf(2.0) * e.powf(2.0)
            + 2.0 * c.powf(2.0) * e.powf(2.0)
            + e.powf(4.0)
            - 2.0 * r2.powf(2.0) * r1.powf(2.0)
            - 2.0 * a.powf(2.0) * r1.powf(2.0)
            - 2.0 * c.powf(2.0) * r1.powf(2.0)
            - 2.0 * e.powf(2.0) * r1.powf(2.0)
            + r1.powf(4.0);
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
        let dx = -8.0 * r2.powf(2.0) * x
            + 4.0 * (r2.powf(2.0) - r1.powf(2.0) + x.powf(2.0) + y.powf(2.0) + z.powf(2.0)) * x;
        let dy = -8.0 * r2.powf(2.0) * y
            + 4.0 * (r2.powf(2.0) - r1.powf(2.0) + x.powf(2.0) + y.powf(2.0) + z.powf(2.0)) * y;
        let dz = 4.0 * (r2.powf(2.0) - r1.powf(2.0) + x.powf(2.0) + y.powf(2.0) + z.powf(2.0)) * z;
        let normal = Vector3::new(dx, dy, dz).normalize();

        Some(Intersection {
            point,
            normal,
            distance: t,
        })
    }

    fn get_aabb(&self) -> AABB {
        //TODO!
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        AABB::new(bln, trf)
    }
}

// GNOMON -----------------------------------------------------------------
#[derive(Clone)]
pub struct Gnonom {
    x_cube: Rc<dyn Primitive>,
    y_cube: Rc<dyn Primitive>,
    z_cube: Rc<dyn Primitive>,
}

impl Gnonom {
    const GNONOM_WIDTH: f64 = 0.1;
    const GNONOM_LENGTH: f64 = 2.0;
    pub fn new() -> Rc<dyn Primitive> {
        let x_cube = Cube::new(
            Point3::new(0.0, -Self::GNONOM_WIDTH, -Self::GNONOM_WIDTH),
            Point3::new(Self::GNONOM_LENGTH, Self::GNONOM_WIDTH, Self::GNONOM_WIDTH),
        );
        let y_cube = Cube::new(
            Point3::new(-Self::GNONOM_WIDTH, 0.0, -Self::GNONOM_WIDTH),
            Point3::new(Self::GNONOM_WIDTH, Self::GNONOM_LENGTH, Self::GNONOM_WIDTH),
        );
        let z_cube = Cube::new(
            Point3::new(-Self::GNONOM_WIDTH, -Self::GNONOM_WIDTH, 0.0),
            Point3::new(Self::GNONOM_WIDTH, Self::GNONOM_WIDTH, Self::GNONOM_LENGTH),
        );
        Rc::new(Gnonom {
            x_cube,
            y_cube,
            z_cube,
        })
    }
}

impl Primitive for Gnonom {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        match self.x_cube.intersect_ray(ray) {
            Some(intersect) => return Some(intersect),
            None => (),
        };
        match self.y_cube.intersect_ray(ray) {
            Some(intersect) => return Some(intersect),
            None => (),
        };
        match self.z_cube.intersect_ray(ray) {
            Some(intersect) => return Some(intersect),
            None => (),
        };
        None
    }

    fn get_aabb(&self) -> AABB {
        AABB::new(
            Point3::new(
                -Self::GNONOM_WIDTH,
                -Self::GNONOM_WIDTH,
                -Self::GNONOM_WIDTH,
            ),
            Point3::new(
                Self::GNONOM_LENGTH,
                Self::GNONOM_LENGTH,
                Self::GNONOM_LENGTH,
            ),
        )
    }
}

// CROSS CAP ---------
#[derive(Clone)]
pub struct CrossCap {}

impl CrossCap {
    pub fn new() -> Rc<dyn Primitive> {
        // I need to find the bounding box for this shape
        Rc::new(CrossCap {})
    }
}

impl Primitive for CrossCap {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;

        let t0 = a.powf(2.0) * c.powf(2.0) + a.powf(2.0) * e.powf(2.0)
            - e.powf(4.0)
            - 2.0 * a * c.powf(2.0)
            - a * e.powf(2.0)
            + c.powf(2.0);
        let t1 = 2.0 * a * b * c.powf(2.0)
            + 2.0 * a.powf(2.0) * c * d
            + 2.0 * a * b * e.powf(2.0)
            + 2.0 * a.powf(2.0) * e * f
            - 4.0 * e.powf(3.0) * f
            - 2.0 * b * c.powf(2.0)
            - 4.0 * a * c * d
            - b * e.powf(2.0)
            - 2.0 * a * e * f
            + 2.0 * c * d;
        let t2 = b.powf(2.0) * c.powf(2.0)
            + 4.0 * a * b * c * d
            + a.powf(2.0) * d.powf(2.0)
            + b.powf(2.0) * e.powf(2.0)
            + 4.0 * a * b * e * f
            + a.powf(2.0) * f.powf(2.0)
            - 6.0 * e.powf(2.0) * f.powf(2.0)
            - 4.0 * b * c * d
            - 2.0 * a * d.powf(2.0)
            - 2.0 * b * e * f
            - a * f.powf(2.0)
            + d.powf(2.0);
        let t3 = 2.0 * b.powf(2.0) * c * d
            + 2.0 * a * b * d.powf(2.0)
            + 2.0 * b.powf(2.0) * e * f
            + 2.0 * a * b * f.powf(2.0)
            - 4.0 * e * f.powf(3.0)
            - 2.0 * b * d.powf(2.0)
            - b * f.powf(2.0);
        let t4 = b.powf(2.0) * d.powf(2.0) + b.powf(2.0) * f.powf(2.0) - f.powf(4.0);

        let t = match match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        } {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = 2.0 * x * y.powf(2.0) + 2.0 * x * z.powf(2.0) - 2.0 * y.powf(2.0) - z.powf(2.0);
        let dy = 2.0 * x.powf(2.0) * y - 4.0 * x * y + 2.0 * y;
        let dz = 2.0 * x.powf(2.0) * z - 4.0 * z.powf(3.0) - 2.0 * x * z;
        let normal = Vector3::new(dx, dy, dz).normalize();

        Some(Intersection {
            point,
            normal,
            distance: t,
        })
    }

    fn get_aabb(&self) -> AABB {
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        AABB::new(bln, trf)
    }
}

// CROSS CAP 2 ---------
#[derive(Clone)]
pub struct CrossCap2 {
    p: f64,
    q: f64,
}

impl CrossCap2 {
    pub fn new(p: f64, q: f64) -> Rc<dyn Primitive> {
        // I need to find the bounding box for this shape
        Rc::new(CrossCap2 { p, q })
    }
}

impl Primitive for CrossCap2 {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;
        let (p, q) = (self.p, self.q);

        let t0 = 2.0 * a * a * e
            + 2.0 * c * c * e
            + a * a * a * a / p
            + a * a * c * c / p
            + a * a * e * e / p
            + a * a * c * c / q
            + c * c * c * c / q
            + c * c * e * e / q;
        let t1 = 4.0 * a * b * e
            + 4.0 * c * d * e
            + 2.0 * a * a * f
            + 2.0 * c * c * f
            + 4.0 * a * a * a * b / p
            + 2.0 * a * b * c * c / p
            + 2.0 * a * a * c * d / p
            + 2.0 * a * b * e * e / p
            + 2.0 * a * a * e * f / p
            + 2.0 * a * b * c * c / q
            + 2.0 * a * a * c * d / q
            + 4.0 * c * c * c * d / q
            + 2.0 * c * d * e * e / q
            + 2.0 * c * c * e * f / q;
        let t2 = 2.0 * b * b * e
            + 2.0 * d * d * e
            + 4.0 * a * b * f
            + 4.0 * c * d * f
            + 6.0 * a * a * b * b / p
            + b * b * c * c / p
            + 4.0 * a * b * c * d / p
            + a * a * d * d / p
            + b * b * e * e / p
            + 4.0 * a * b * e * f / p
            + a * a * f * f / p
            + b * b * c * c / q
            + 4.0 * a * b * c * d / q
            + a * a * d * d / q
            + 6.0 * c * c * d * d / q
            + d * d * e * e / q
            + 4.0 * c * d * e * f / q
            + c * c * f * f / q;
        let t3 = 2.0 * b * b * f
            + 2.0 * d * d * f
            + 4.0 * a * b * b * b / p
            + 2.0 * b * b * c * d / p
            + 2.0 * a * b * d * d / p
            + 2.0 * b * b * e * f / p
            + 2.0 * a * b * f * f / p
            + 2.0 * b * b * c * d / q
            + 2.0 * a * b * d * d / q
            + 4.0 * c * d * d * d / q
            + 2.0 * d * d * e * f / q
            + 2.0 * c * d * f * f / q;
        let t4 = b * b * b * b / p
            + b * b * d * d / p
            + b * b * f * f / p
            + b * b * d * d / q
            + d * d * d * d / q
            + d * d * f * f / q;
        let t = match match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        } {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx =
            2.0 * x * (x * (2.0 / p) + y * (2.0 / q)) + 4.0 * x * z + 2.0 * (x * (x + y + z) / p);
        let dy =
            2.0 * (x * (2.0 / p) + y * (2.0 / q)) * y + 4.0 * y * z + 2.0 * (y * (x + y + z) / q);
        let dz = 2.0 * x + 2.0 * y + 2.0 * (x * (2.0 / p) + y * (2.0 / q)) * z;
        let normal = Vector3::new(dx, dy, dz).normalize();

        Some(Intersection {
            point,
            normal,
            distance: t,
        })
    }

    fn get_aabb(&self) -> AABB {
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        AABB::new(bln, trf)
    }
}

//  Steiner  ---------
#[derive(Clone)]
pub struct Steiner {}

impl Steiner {
    pub fn new() -> Rc<dyn Primitive> {
        // I need to find the bounding box for this shape
        Rc::new(Steiner {})
    }
}

impl Primitive for Steiner {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;
        let t0 = a.powf(2.0) * c.powf(2.0) - a.powf(2.0) * e.powf(2.0) + c.powf(2.0) * e.powf(2.0)
            - a * c * e;
        let t1 = 2.0 * a * b * c.powf(2.0) + 2.0 * a.powf(2.0) * c * d - 2.0 * a * b * e.powf(2.0)
            + 2.0 * c * d * e.powf(2.0)
            - 2.0 * a.powf(2.0) * e * f
            + 2.0 * c.powf(2.0) * e * f
            - b * c * e
            - a * d * e
            - a * c * f;
        let t2 = b.powf(2.0) * c.powf(2.0) + 4.0 * a * b * c * d + a.powf(2.0) * d.powf(2.0)
            - b.powf(2.0) * e.powf(2.0)
            + d.powf(2.0) * e.powf(2.0)
            - 4.0 * a * b * e * f
            + 4.0 * c * d * e * f
            - a.powf(2.0) * f.powf(2.0)
            + c.powf(2.0) * f.powf(2.0)
            - b * d * e
            - b * c * f
            - a * d * f;
        let t3 = 2.0 * b.powf(2.0) * c * d + 2.0 * a * b * d.powf(2.0) - 2.0 * b.powf(2.0) * e * f
            + 2.0 * d.powf(2.0) * e * f
            - 2.0 * a * b * f.powf(2.0)
            + 2.0 * c * d * f.powf(2.0)
            - b * d * f;
        let t4 = b.powf(2.0) * d.powf(2.0) - b.powf(2.0) * f.powf(2.0) + d.powf(2.0) * f.powf(2.0);

        let t = match match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        } {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = 2.0 * x * y.powf(2.0) - 2.0 * x * z.powf(2.0) - y * z;
        let dy = 2.0 * x.powf(2.0) * y + 2.0 * y * z.powf(2.0) - x * z;
        let dz = -2.0 * x.powf(2.0) * z + 2.0 * y.powf(2.0) * z - x * y;
        let normal = Vector3::new(dx, dy, dz).normalize();

        Some(Intersection {
            point,
            normal,
            distance: t,
        })
    }

    fn get_aabb(&self) -> AABB {
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        AABB::new(bln, trf)
    }
}

//  Steiner 2 ---------
#[derive(Clone)]
pub struct Steiner2 {}

impl Steiner2 {
    pub fn new() -> Rc<dyn Primitive> {
        // I need to find the bounding box for this shape
        Rc::new(Steiner2 {})
    }
}

impl Primitive for Steiner2 {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;

        let t0 = a.powi(2) * c.powi(2) + a.powi(2) * e.powi(2)
            - e.powi(4)
            - 2.0 * a * c.powi(2)
            - a * e.powi(2)
            + c.powi(2);
        let t1 = 2.0 * a * b * c.powi(2)
            + 2.0 * a.powi(2) * c * d
            + 2.0 * a * b * e.powi(2)
            + 2.0 * a.powi(2) * e * f
            - 4.0 * e.powi(3) * f
            - 2.0 * b * c.powi(2)
            - 4.0 * a * c * d
            - b * e.powi(2)
            - 2.0 * a * e * f
            + 2.0 * c * d;
        let t2 = b.powi(2) * c.powi(2)
            + 4.0 * a * b * c * d
            + a.powi(2) * d.powi(2)
            + b.powi(2) * e.powi(2)
            + 4.0 * a * b * e * f
            + a.powi(2) * f.powi(2)
            - 6.0 * e.powi(2) * f.powi(2)
            - 4.0 * b * c * d
            - 2.0 * a * d.powi(2)
            - 2.0 * b * e * f
            - a * f.powi(2)
            + d.powi(2);
        let t3 = 2.0 * b.powi(2) * c * d
            + 2.0 * a * b * d.powi(2)
            + 2.0 * b.powi(2) * e * f
            + 2.0 * a * b * f.powi(2)
            - 4.0 * e * f.powi(3)
            - 2.0 * b * d.powi(2)
            - b * f.powi(2);
        let t4 = b.powi(2) * d.powi(2) + b.powi(2) * f.powi(2) - f.powi(4);

        let t = match match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        } {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = 2.0 * x * y.powi(2) + 2.0 * x * z.powi(2) - 2.0 * y.powi(2) - z.powi(2);
        let dy = 2.0 * x.powi(2) * y - 4.0 * x * y + 2.0 * y;
        let dz = 2.0 * x.powi(2) * z - 4.0 * z.powi(3) - 2.0 * x * z;
        let normal = Vector3::new(dx, dy, dz).normalize();

        Some(Intersection {
            point,
            normal,
            distance: t,
        })
    }

    fn get_aabb(&self) -> AABB {
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        AABB::new(bln, trf)
    }
}

//  Roman  ---------
#[derive(Clone)]
pub struct Roman {
    k: f64,
}

impl Roman {
    pub fn new(k: f64) -> Rc<dyn Primitive> {
        // I need to find the bounding box for this shape
        Rc::new(Roman { k })
    }
}

impl Primitive for Roman {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.a.x;
        let b = ray.b.x;
        let c = ray.a.y;
        let d = ray.b.y;
        let e = ray.a.z;
        let f = ray.b.z;
        let k = self.k;

        let t0 = a.powf(4.0)
            + 2.0 * a.powf(2.0) * c.powf(2.0)
            + c.powf(4.0)
            + 2.0 * a.powf(2.0) * e.powf(2.0)
            + 2.0 * c.powf(2.0) * e.powf(2.0)
            + e.powf(4.0)
            - 2.0 * a.powf(2.0) * k.powf(2.0)
            - 2.0 * c.powf(2.0) * k.powf(2.0)
            - 2.0 * e.powf(2.0) * k.powf(2.0)
            + k.powf(4.0);
        let t1 = 4.0 * a.powf(3.0) * b
            + 4.0 * a * b * c.powf(2.0)
            + 4.0 * a.powf(2.0) * c * d
            + 4.0 * c.powf(3.0) * d
            + 4.0 * a * b * e.powf(2.0)
            + 4.0 * c * d * e.powf(2.0)
            + 4.0 * a.powf(2.0) * e * f
            + 4.0 * c.powf(2.0) * e * f
            + 4.0 * e.powf(3.0) * f
            - 4.0 * a * b * k.powf(2.0)
            - 4.0 * c * d * k.powf(2.0)
            - 4.0 * e * f * k.powf(2.0);
        let t2 = 6.0 * a.powf(2.0) * b.powf(2.0)
            + 2.0 * b.powf(2.0) * c.powf(2.0)
            + 8.0 * a * b * c * d
            + 2.0 * a.powf(2.0) * d.powf(2.0)
            + 6.0 * c.powf(2.0) * d.powf(2.0)
            + 2.0 * b.powf(2.0) * e.powf(2.0)
            + 2.0 * d.powf(2.0) * e.powf(2.0)
            + 8.0 * a * b * e * f
            + 8.0 * c * d * e * f
            + 2.0 * a.powf(2.0) * f.powf(2.0)
            + 2.0 * c.powf(2.0) * f.powf(2.0)
            + 6.0 * e.powf(2.0) * f.powf(2.0)
            - 2.0 * b.powf(2.0) * k.powf(2.0)
            - 2.0 * d.powf(2.0) * k.powf(2.0)
            - 2.0 * f.powf(2.0) * k.powf(2.0);
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
        let t = match match find_roots_quartic(t4, t3, t2, t1, t0) {
            Roots::No(arr) => smallest_non_zero(&arr),
            Roots::One(arr) => smallest_non_zero(&arr),
            Roots::Two(arr) => smallest_non_zero(&arr),
            Roots::Three(arr) => smallest_non_zero(&arr),
            Roots::Four(arr) => smallest_non_zero(&arr),
        } {
            Some(t) => t,
            None => return None,
        };

        let point = ray.at_t(t);
        let (x, y, z) = (point.x, point.y, point.z);
        let dx = -4.0 * (k.powf(2.0) - x.powf(2.0) - y.powf(2.0) - z.powf(2.0)) * x;
        let dy = -4.0 * (k.powf(2.0) - x.powf(2.0) - y.powf(2.0) - z.powf(2.0)) * y;
        let dz = -4.0 * (k.powf(2.0) - x.powf(2.0) - y.powf(2.0) - z.powf(2.0)) * z;
        let normal = Vector3::new(dx, dy, dz).normalize();

        Some(Intersection {
            point,
            normal,
            distance: t,
        })
    }

    fn get_aabb(&self) -> AABB {
        let trf = Point3::new(1.0, 1.0, 1.0);
        let bln = Point3::new(-1.0, -1.0, -1.0);
        AABB::new(bln, trf)
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
