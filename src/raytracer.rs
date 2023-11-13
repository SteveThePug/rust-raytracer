use crate::{
    primitive::{Intersection, Primitive},
    ray::Ray,
    scene::Scene,
    INFINITY,
};
use nalgebra::{distance, Matrix4, Point3, Vector3, Vector4};

// Find the closest intersection, given a ray in world coordinates
fn get_closest_intersection<'a>(scene: &'a Scene, ray: &Ray) -> Option<Intersection<'a>> {
    let mut closest_distance = INFINITY;
    let mut closest_intersect: Option<Intersection> = None;
    for primitive in &scene.primitives {
        if primitive.intersect_bounding_box(ray) == None {
            continue;
        };

        let intersect = primitive.intersect_ray(ray);
        if intersect.is_none() {
            continue;
        };

        let intersect = intersect.unwrap();
        if intersect.distance < closest_distance {
            closest_distance = intersect.distance;
            closest_intersect = Some(intersect);
        }
    }
    closest_intersect
}

// We want to shade a point placed in our scene
fn phong_shade_point(scene: &Scene, intersect: &Intersection) -> Option<Vector4<f32>> {
    //Unpack the intersection data
    let Intersection {
        primitive,
        point,
        normal,
        incidence,
        distance,
    } = intersect;
    let ambient_light = scene.ambient_light;
    let material = primitive.get_material();
    let kd = material.kd;
    let ks = material.kd;
    let shininess = material.shininess;
    // We should now have all the information for our ray-tracer

    // Let us first compute the ambient light component
    let ambient = ambient_light * kd;
    None
}
