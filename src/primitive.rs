use crate::ray::Ray;
use crate::EPSILON;
use nalgebra::{distance, Matrix4, Point3, Vector3};
use roots::{find_roots_quadratic, Roots};
// MATERIAL -----------------------------------------------------------------
struct Material {
    kd: Vector3<f32>,
    ks: Vector3<f32>,
    shininess: f32,
}
impl Material {
    fn new(kd: Vector3<f32>, ks: Vector3<f32>, shininess: f32) -> Self {
        Material { kd, ks, shininess }
    }
    fn magenta() -> Self {
        let kd = Vector3::new(1.0, 0.0, 1.0);
        let ks = Vector3::new(0.0, 1.0, 1.0);
        let shininess = 0.5;
        Material { kd, ks, shininess }
    }
}
// INTERSECTION -----------------------------------------------------------------
struct Intersection {
    point: Point3<f32>,
    normal: Vector3<f32>,
    // Information about an intersection
}
impl Intersection {
    fn new(point: Point3<f32>, normal: Vector3<f32>) -> Self {
        Intersection { point, normal }
    }
}
// BOUNDING BOX -----------------------------------------------------------------
struct BoundingBox {
    bln: Point3<f32>,
    trf: Point3<f32>,
}
impl BoundingBox {
    fn new(bln: Point3<f32>, trf: Point3<f32>) -> Self {
        BoundingBox { bln, trf }
    }
    fn intersect_bounding_box(
        &self,
        position: &Vector3<f32>,
        direction: &Vector3<f32>,
    ) -> Option<&Self> {
        unimplemented!()
    }
    fn distance_to_point(&self, point: &Vector3<f32>) -> f32 {
        unimplemented!()
    }
    fn update(&self, bln: Point3<f32>, trf: Point3<f32>) {
        unimplemented!()
    }
}
// PRIMITIVE TRAIT -----------------------------------------------------------------
trait Primitive {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
    fn get_material(self) -> Material;
}

// SPHERE -----------------------------------------------------------------
struct Sphere {
    position: Point3<f32>,
    radius: f32,
    material: Material,
    bounding_box: BoundingBox,
}

impl Sphere {
    fn new(position: Point3<f32>, radius: f32, material: Material) -> Self {
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        let bounding_box = BoundingBox::new(bln, trf);
        Sphere {
            position,
            radius,
            material,
            bounding_box,
        }
    }

    fn unit() -> Self {
        let position = Point3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        let radius_vec = Vector3::new(radius, radius, radius);
        let material = Material::magenta();
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        let bounding_box = BoundingBox { bln, trf };
        Sphere {
            position,
            radius,
            material,
            bounding_box,
        }
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
        let normal = (intersect - self.position).normalize();
        Some(Intersection {
            point: intersect,
            normal,
        })
    }

    fn get_material(self) -> Material {
        self.material
    }
}

// CIRCLE -----------------------------------------------------------------
struct Circle {
    position: Point3<f32>,
    radius: f32,
    normal: Vector3<f32>,
    material: Material,
    bounding_box: BoundingBox,
}

impl Circle {
    fn new(position: Point3<f32>, radius: f32, normal: Vector3<f32>, material: Material) -> Self {
        let normal = normal.normalize();
        let radius_vec = Vector3::new(radius, radius, radius);
        let bln = position - radius_vec;
        let trf = position + radius_vec;
        let bounding_box = BoundingBox::new(bln, trf);
        Circle {
            position,
            radius,
            normal,
            material,
            bounding_box,
        }
    }

    fn unit() -> Self {
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let radius = 1.0;
        let material = Material::magenta();

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
        let intersect = ray.at_t(t);
        let distance = distance(&intersect, &self.position);
        match distance > self.radius {
            true => return None,
            false => {
                return Some(Intersection {
                    point: intersect,
                    normal: self.normal,
                })
            }
        }
    }

    fn get_material(self) -> Material {
        self.material
    }
}

// CYLINDER -----------------------------------------------------------------
struct Cylinder {}

impl Cylinder {}

impl Primitive for Cylinder {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }

    fn get_material(self) -> Material {
        todo!()
    }
}

// CONE -----------------------------------------------------------------
struct Cone {
    radius: f32,
    height: f32,
    base: f32,
    circle: Circle,
}

impl Cone {
    fn get_normal(&self, intersect: Point3<f32>) -> Vector3<f32> {
        let r = self.radius;
        let h = self.height;
        let (x, y, z) = (intersect.x, intersect.y, intersect.z);
        let normal = Vector3::new(2.0 * x, 2.0 * r * r * (h - y), 2.0 * z).normalize();
        return normal;
    }
}

impl Primitive for Cone {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let point = &ray.a;
        let dir = &ray.b;
        let (r, h) = (self.radius, self.height);
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
                match intersect.y >= self.base && intersect.y <= self.height {
                    true => Some(Intersection {
                        point: intersect,
                        normal: self.get_normal(intersect),
                    }),
                    false => None,
                }
            }
        };

        let circle_intersect = self.circle.intersect_ray(ray);

        match (cone_intersect, circle_intersect) {
            (None, None) => return None,
            (Some(cone_intersect), None) => return Some(cone_intersect),
            (None, Some(circle_intersect)) => return Some(circle_intersect),
            (Some(cone_intersect), Some(circle_intersect)) => {
                let circle_distance = distance(&ray.a, &circle_intersect.point);
                let cone_distance = distance(&ray.a, &cone_intersect.point);
                match cone_distance < circle_distance {
                    true => return Some(cone_intersect),
                    false => return Some(circle_intersect),
                }
            }
        };
    }

    fn get_material(self) -> Material {
        todo!()
    }
}

// BOX -----------------------------------------------------------------
struct Box {}
impl Box {}
impl Primitive for Box {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }

    fn get_material(self) -> Material {
        todo!()
    }
}

// TRIANGLE -----------------------------------------------------------------
struct Triangle {}
impl Triangle {}
impl Primitive for Triangle {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }

    fn get_material(self) -> Material {
        todo!()
    }
}

// MESH -----------------------------------------------------------------
struct Mesh {}
impl Mesh {}
impl Primitive for Mesh {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }

    fn get_material(self) -> Material {
        todo!()
    }
}
