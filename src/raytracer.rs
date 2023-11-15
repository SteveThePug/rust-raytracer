use crate::{
    light::Light,
    primitive::{Intersection, Primitive},
    ray::Ray,
    scene::Scene,
    INFINITY,
};
use std::sync::Arc;

use nalgebra::{distance, Matrix4, Point3, Vector3, Vector4};

pub fn shade_rays(scene: &Scene, rays: &Vec<Ray>, width: u32, height: u32) -> Vec<Vector3<u8>> {
    let mut pixel_data = vec![];

    for ray in rays {
        let intersect = get_closest_intersection(scene, ray);
        match intersect {
            Some(interect) => {
                let colour = phong_shade_point(scene, &interect);
                pixel_data.push(colour);
            }
            None => {
                let colour = Vector3::new(0, 0, 0);
                pixel_data.push(colour);
            }
        }
    }

    pixel_data
}

// Find the closest intersection, given a ray in world coordinates
pub fn get_closest_intersection(scene: &Scene, ray: &Ray) -> Option<Intersection> {
    let mut closest_distance = INFINITY;
    let mut closest_intersect: Option<Intersection> = None;
    for arc_primitive in &scene.primitives {
        let primitive = arc_primitive.clone();

        if primitive.intersect_ray(ray).is_none() {
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
pub fn phong_shade_point(scene: &Scene, intersect: &Intersection) -> Vector3<u8> {
    //Useful vectors !!!! CHECK IF WE CAN OPTIMISE
    let zero_vector = Vector3::new(0.0, 0.0, 0.0);
    let one_vector = Vector3::new(1.0, 1.0, 1.0);
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
    let r = nalgebra::clamp(colour.x * 255.0, 0.0, 255.0) as u8;
    let g = nalgebra::clamp(colour.y * 255.0, 0.0, 255.0) as u8;
    let b = nalgebra::clamp(colour.z * 255.0, 0.0, 255.0) as u8;

    Vector3::new(r, g, b)
}
