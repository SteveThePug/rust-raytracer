use crate::{light::Light, primitive::Intersection, ray::Ray, scene::*, EPSILON, ZERO_VECTOR};

use nalgebra::{Unit, Vector3};

// Function to shade a point in the scene using Phong shading model
pub fn phong_shade_point(scene: &Scene, intersect: &Intersection) -> Vector3<u8> {
    let Intersection {
        point,
        normal,
        incidence,
        material,
        ..
    } = intersect;

    let kd = material.kd;
    let ks = material.ks;
    let shininess = material.shininess;

    // Compute the ambient light component and set it as base colour
    let mut colour = ZERO_VECTOR;

    for light in &scene.lights {
        let Light {
            position: light_position,
            colour: light_colour,
            falloff: light_falloff,
        } = light;

        // Point to light
        let to_light = light_position - point;
        let light_distance = to_light.norm();
        let to_light = to_light;

        let to_light_ray = Ray::new(point.clone() + normal * EPSILON, to_light);
        if light_blocked(scene, to_light_ray) {
            continue;
        }

        // Point to camera
        let to_camera = -incidence;
        // Diffuse component
        let n_dot_l = normal.dot(&to_light).max(0.0);
        let diffuse = n_dot_l * kd;
        // Specular component
        let mut specular = ZERO_VECTOR;
        if n_dot_l > 0.0 {
            // Halfway vector.
            let h = Unit::new_normalize(to_camera.lerp(&to_light, 0.5));
            let n_dot_h = normal.dot(&h).max(0.0);
            specular = ks * n_dot_h.powf(shininess);
        }

        // Compute light falloff
        let falloff = 1.0
            / (1.0
                + light_falloff[0]
                + light_falloff[1] * light_distance
                + light_falloff[2] * light_distance.powi(2));

        let light_intensity = light_colour.component_mul(&(diffuse + specular)) * falloff;
        colour += &light_intensity;
    }

    colour *= 255.0;
    let (r, g, b) = (colour.x as u8, colour.y as u8, colour.z as u8);
    Vector3::new(r, g, b)
}

fn light_blocked(scene: &Scene, ray: Ray) -> bool {
    for node in &scene.nodes {
        let ray = ray.transform(&node.inv_model);
        if node.primitive.intersect_bounding_box(&ray).is_some() {
            if node.primitive.intersect_ray(&ray).is_some() {
                return true;
            }
        }
    }
    false
}
