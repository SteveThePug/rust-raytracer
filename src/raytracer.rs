use crate::{
    light::Light,
    primitive::{Intersection, Primitive},
    ray::Ray,
    scene::Scene,
    INFINITY,
};
use lazy_static::lazy_static;
lazy_static! {
    static ref VEC3_ONE: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
    static ref VEC3_ZERO: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
}

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
fn phong_shade_point(scene: &Scene, intersect: &Intersection) -> Vector3<f32> {
    //Useful vectors !!!! CHECK IF WE CAN OPTIMISE
    let zero_vector = Vector3::new(0.0, 0.0, 0.0);
    let one_vector = Vector3::new(1.0, 1.0, 1.0);
    //Unpack the intersection data
    let Intersection {
        primitive,
        point,
        normal,
        incidence,
        ..
    } = intersect;
    let ambient_light = scene.ambient_light;
    let material = primitive.get_material();
    let kd = material.kd;
    let ks = material.ks;
    let shininess = material.shininess;
    // We should now have all the information for our ray-tracer

    // Let us first compute the ambient light component and set it as out base colour
    let mut colour = kd.component_mul(&ambient_light);

    for light in &scene.lights {
        let Light {
            position: light_position,
            colour: light_colour,
            falloff: light_falloff,
        } = light;

        // Get light incidence vector
        let to_light = light_position - point;
        let light_distance = to_light.norm();
        let light_incidence = to_light.normalize();

        // Compute light falloff
        let falloff = 1.0
            / (light_falloff[0]
                + light_falloff[1] * light_distance
                + light_falloff[2] * light_distance * light_distance);

        // Compute diffuse
        let n_dot_l = normal.dot(&light_incidence);
        let diffuse = if n_dot_l > 0.0 {
            kd * n_dot_l
        } else {
            zero_vector
        };

        // Compute specular
        let h = (light_incidence + incidence).normalize();
        let n_dot_h = normal.dot(&h);
        let specular = if n_dot_h > 0.0 {
            ks * n_dot_h.powf(shininess)
        } else {
            zero_vector
        };

        colour += light_colour.component_mul(&((diffuse + specular) * falloff));
    }
    nalgebra::clamp(colour, zero_vector, one_vector)
}
