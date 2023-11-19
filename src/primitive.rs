use crate::ray::Ray;
use crate::{EPSILON, EPSILON_VECTOR, INFINITY};
use nalgebra::{distance, Matrix4, Point3, Unit, Vector3};
use roots::{find_roots_quadratic, Roots};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

// MATERIAL -----------------------------------------------------------------
#[derive(Clone)]
pub struct Material {
    pub kd: Vector3<f32>,
    pub ks: Vector3<f32>,
    pub shininess: f32,
}
impl Material {
    pub fn new(kd: Vector3<f32>, ks: Vector3<f32>, shininess: f32) -> Self {
        Material { kd, ks, shininess }
    }
    pub fn magenta() -> Self {
        let kd = Vector3::new(1.0, 0.0, 1.0);
        let ks = Vector3::new(1.0, 0.0, 1.0);
        let shininess = 0.5;
        Material { kd, ks, shininess }
    }
    pub fn turquoise() -> Self {
        let kd = Vector3::new(0.25, 0.3, 0.7);
        let ks = Vector3::new(0.25, 0.3, 0.7);
        let shininess = 0.5;
        Material { kd, ks, shininess }
    }
    pub fn red() -> Self {
        let kd = Vector3::new(0.8, 0.0, 0.3);
        let ks = Vector3::new(0.8, 0.3, 0.0);
        let shininess = 0.5;
        Material { kd, ks, shininess }
    }
    pub fn blue() -> Self {
        let kd = Vector3::new(0.0, 0.3, 0.6);
        let ks = Vector3::new(0.3, 0.0, 0.6);
        let shininess = 0.5;
        Material { kd, ks, shininess }
    }
    pub fn green() -> Self {
        let kd = Vector3::new(0.0, 1.0, 0.0);
        let ks = Vector3::new(0.0, 1.0, 0.0);
        let shininess = 0.5;
        Material { kd, ks, shininess }
    }
}
// INTERSECTION -----------------------------------------------------------------
pub struct Intersection {
    // Information about an intersection
    pub point: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub incidence: Unit<Vector3<f32>>,
    pub material: Arc<Material>,
    pub distance: f32,
}
impl Intersection {
    pub fn new(
        point: Point3<f32>,
        normal: Unit<Vector3<f32>>,
        incidence: Unit<Vector3<f32>>,
        material: Arc<Material>,
        t: f32,
    ) -> Self {
        Intersection {
            point,
            normal,
            incidence,
            material,
            distance: t,
        }
    }
}
// BOUNDING BOX -----------------------------------------------------------------
#[derive(Clone)]
struct BoundingBox {
    bln: Point3<f32>,
    trf: Point3<f32>,
}

impl BoundingBox {
    fn new(bln: Point3<f32>, trf: Point3<f32>) -> Self {
        let bln = bln - EPSILON_VECTOR;
        let trf = trf + EPSILON_VECTOR;
        BoundingBox { bln, trf }
    }
    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
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
    fn get_centroid(&self) -> Point3<f32> {
        self.bln + (self.trf - self.bln) / 2.0
    }
}
// PRIMITIVE TRAIT -----------------------------------------------------------------
pub trait Primitive: Send + Sync {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>>;
    fn get_material(&self) -> Arc<Material>;
}

// SPHERE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Sphere {
    position: Point3<f32>,
    radius: f32,
    bounding_box: BoundingBox,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new(position: Point3<f32>, radius: f32, material: Arc<Material>) -> Self {
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        let bounding_box = BoundingBox::new(bln, trf);
        Sphere {
            position,
            radius,
            bounding_box,
            material,
        }
    }

    pub fn unit(material: Arc<Material>) -> Self {
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

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        return self.bounding_box.intersect_bounding_box(ray);
    }
}

// CIRCLE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Circle {
    position: Point3<f32>,
    radius: f32,
    normal: Vector3<f32>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Circle {
    pub fn new(
        position: Point3<f32>,
        radius: f32,
        normal: Vector3<f32>,
        material: Arc<Material>,
    ) -> Self {
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        let bounding_box = BoundingBox::new(bln, trf);
        Circle {
            position,
            radius,
            normal: normal.normalize(),
            material,
            bounding_box,
        }
    }

    pub fn unit(material: Arc<Material>) -> Self {
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let radius = 1.0;
        let material = material;

        let bln = Point3::new(-radius, 0.0, -EPSILON);
        let trf = Point3::new(radius, 0.0, EPSILON);
        let bounding_box = BoundingBox { bln, trf };

        Circle {
            position,
            normal,
            radius,
            material,
            bounding_box,
        }
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

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// CYLINDER -----------------------------------------------------------------
struct Cylinder {
    radius: f32,
    base: f32,
    height: f32,
    base_circle: Circle,
    height_circle: Circle,
    material: Arc<Material>,
}

impl Cylinder {}

impl Primitive for Cylinder {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }

    fn get_material(&self) -> Arc<Material> {
        todo!()
    }

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        todo!()
    }
}

// CONE -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cone {
    radius: f32,
    base: f32,
    apex: f32,
    circle: Circle,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Cone {
    pub fn new(radius: f32, apex: f32, base: f32, material: Arc<Material>) -> Self {
        let circle = Circle::new(
            Point3::new(0.0, base, 0.0),
            radius,
            Vector3::new(0.0, 1.0, 0.0),
            Arc::clone(&material),
        );
        let bln = Point3::new(-radius, base, -radius);
        let trf = Point3::new(radius, base + apex, radius);
        Cone {
            radius: radius / 2.0,
            base,
            apex,
            circle,
            material,
            bounding_box: BoundingBox { bln, trf },
        }
    }
    pub fn unit(material: Arc<Material>) -> Self {
        Cone::new(1.0, 2.0, -1.0, material)
    }

    pub fn get_normal(&self, intersect: Point3<f32>) -> Vector3<f32> {
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

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// RECTANGLE -----------------------------------------------------------------
struct Rectangle {
    position: Point3<f32>,
    normal: Vector3<f32>,
    width_direction: Vector3<f32>,
    material: Arc<Material>,
    width: f32,
    height: f32,
    bounding_box: BoundingBox,
}

impl Rectangle {
    fn new(
        position: Point3<f32>,
        normal: Vector3<f32>,
        width_direction: Vector3<f32>,
        width: f32,
        height: f32,
        material: Arc<Material>,
    ) -> Self {
        let normal = normal.normalize();
        let width_direction = width_direction.normalize();
        let height_direction = width_direction.cross(&normal);
        let bln = position - width / 2.0 * width_direction - height / 2.0 * height_direction;
        let trf = position + width / 2.0 * width_direction + height / 2.0 * height_direction;
        Rectangle {
            position,
            normal: normal.normalize(),
            width_direction: width_direction.normalize(),
            width,
            height,
            material,
            bounding_box: BoundingBox { bln, trf },
        }
    }
    pub fn unit(material: Arc<Material>) -> Self {
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

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// BOX -----------------------------------------------------------------
#[derive(Clone)]
pub struct Cube {
    width: f32,
    height: f32,
    depth: f32,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Cube {
    fn new(width: f32, height: f32, depth: f32, material: Arc<Material>) -> Self {
        let trf = Point3::new(width / 2.0, height / 2.0, depth / 2.0);
        let bln = Point3::new(-width / 2.0, -height / 2.0, -depth / 2.0);
        Cube {
            width,
            height,
            depth,
            material,
            bounding_box: BoundingBox { bln, trf },
        }
    }
    pub fn unit(material: Arc<Material>) -> Self {
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

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        self.bounding_box.intersect_bounding_box(ray)
    }

    fn get_material(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}

// TRIANGLE -----------------------------------------------------------------
struct Triangle {
    u: Point3<f32>,
    v: Point3<f32>,
    w: Point3<f32>,
    normal: Vector3<f32>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Triangle {
    fn new(u: Point3<f32>, v: Point3<f32>, w: Point3<f32>, material: Arc<Material>) -> Self {
        let uv = v - u;
        let uw = w - u;
        let normal = uv.cross(&uw).normalize();
        let bln = u.inf(&v).inf(&w);
        let trf = u.sup(&v).sup(&w);
        let bounding_box = BoundingBox { bln, trf };
        Triangle {
            u,
            v,
            w,
            normal,
            material,
            bounding_box,
        }
    }
    pub fn unit(material: Arc<Material>) -> Self {
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

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}

// MESH -----------------------------------------------------------------
struct Mesh {
    triangles: Vec<Triangle>,
    material: Arc<Material>,
    bounding_box: BoundingBox,
}

impl Mesh {
    fn new(triangles: Vec<Triangle>, material: Arc<Material>) -> Self {
        // Calculate the bounding box for the entire mesh based on the bounding boxes of individual triangles
        let bounding_box = Mesh::compute_bounding_box(&triangles);

        Mesh {
            triangles,
            material,
            bounding_box,
        }
    }

    fn compute_bounding_box(triangles: &Vec<Triangle>) -> BoundingBox {
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

    fn from_file(filename: &str, material: Arc<Material>) -> Self {
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut vertices: Vec<Point3<f32>> = Vec::new();

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
                                let x: f32 = x_str.parse().expect("Failed to parse vertex X");
                                let y: f32 = y_str.parse().expect("Failed to parse vertex Y");
                                let z: f32 = z_str.parse().expect("Failed to parse vertex Z");
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

    fn intersect_bounding_box(&self, ray: &Ray) -> Option<Point3<f32>> {
        self.bounding_box.intersect_bounding_box(ray)
    }
}
