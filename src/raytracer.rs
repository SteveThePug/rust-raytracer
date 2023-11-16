use crate::{
    light::Light,
    primitive::{Intersection, Primitive},
    ray::Ray,
    scene::Scene,
    INFINITY,
};
use std::sync::Arc;

use nalgebra::{distance, Matrix4, Point3, Vector3, Vector4};
static ZERO_VECTOR: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
static ONE_VECTOR: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);

pub fn shade_rays(scene: &Scene, rays: &Vec<Ray>, width: i32, height: i32) -> Vec<Vector3<u8>> {
    let mut pixel_data = vec![Vector3::new(0, 0, 0); (width * height) as usize];

    for (pixel_index, ray) in rays.iter().enumerate() {
        let intersect = get_closest_intersection(scene.primitives.clone(), ray);
        let colour = match intersect {
            Some(intersect) => phong_shade_point(scene, &intersect),
            None => {
                // Handle rays that miss objects (e.g., use a background color or environment map)
                Vector3::new(0, 0, 0)
            }
        };
        pixel_data[pixel_index] = colour;
    }
    pixel_data
}
//Shade a single ray
pub fn shade_ray(scene: &Scene, ray: &Ray) -> Vector3<u8> {
    let intersect = get_closest_intersection(scene.primitives.clone(), ray);
    match intersect {
        Some(intersect) => phong_shade_point(&scene, &intersect),
        None => Vector3::new(0, 0, 0),
    }
}

// Find the closest intersection, given a ray in world coordinates
pub fn get_closest_intersection(
    primitives: Vec<Arc<dyn Primitive>>,
    ray: &Ray,
) -> Option<Intersection> {
    let mut closest_distance = INFINITY;
    let mut closest_intersect: Option<Intersection> = None;

    for arc_primitive in primitives {
        let primitive = arc_primitive.clone();

        if let Some(intersect) = primitive.intersect_ray(ray) {
            if intersect.distance < closest_distance {
                closest_distance = intersect.distance;
                closest_intersect = Some(intersect);
            }
        }
    }

    closest_intersect
}

// We want to shade a point placed in our scene
pub fn phong_shade_point(scene: &Scene, intersect: &Intersection) -> Vector3<u8> {
    //Useful vectors !!!! CHECK IF WE CAN OPTIMISE
    //Unpack the intersection data
    let Intersection {
        point,
        normal,
        incidence,
        material,
        ..
    } = intersect;
    let binding = scene.ambient_light.clone();
    let ambient_light = binding.as_ref();
    let kd = material.kd;
    let ks = material.ks;
    let shininess = material.shininess;
    // We should now have all the information for our ray-tracer

    // Let us first compute the ambient light component and set it as out base colour
    let mut colour = kd.component_mul(ambient_light);

    for arc_light in &scene.lights {
        let light = arc_light.clone();

        let Light {
            position: light_position,
            colour: light_colour,
            falloff: light_falloff,
        } = light.as_ref();

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
            ZERO_VECTOR
        };

        // Compute specular
        let h = (light_incidence + incidence).normalize();
        let n_dot_h = normal.dot(&h);
        let specular = if n_dot_h > 0.0 {
            ks * n_dot_h.powf(shininess)
        } else {
            ZERO_VECTOR
        };

        colour += light_colour.component_mul(&((diffuse + specular) * falloff));
    }
    let r = nalgebra::clamp(colour.x * 255.0, 0.0, 255.0) as u8;
    let g = nalgebra::clamp(colour.y * 255.0, 0.0, 255.0) as u8;
    let b = nalgebra::clamp(colour.z * 255.0, 0.0, 255.0) as u8;

    Vector3::new(r, g, b)
}
