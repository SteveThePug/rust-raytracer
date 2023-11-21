use crate::{light::Light, primitive::Intersection, scene::*};

use nalgebra::{Unit, Vector3};

static ZERO_VECTOR: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

// Function to shade a point in the scene using Phong shading model
pub fn phong_shade_point(scene: &Scene, intersect: &Intersection) -> Vector3<u8> {
    let Intersection {
        point,
        normal,
        incidence,
        material,
        ..
    } = intersect;
    let ambient_light = &scene.ambient_light;
    let kd = material.kd;
    let ks = material.ks;
    let shininess = material.shininess;

    // Compute the ambient light component and set it as base colour
    let mut colour = kd.component_mul(ambient_light);

    for light in &scene.lights {
        let Light {
            position: light_position,
            colour: light_colour,
            falloff: light_falloff,
        } = light;

        // Compute light incidence vector and its distance
        let to_light = light_position - point;
        let light_distance = to_light.norm();
        let light_incidence = Unit::new_normalize(to_light);

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
        let h = (&light_incidence.into_inner() + incidence.into_inner()).normalize();
        let n_dot_h = normal.dot(&h);
        let specular = if n_dot_h > 0.0 {
            ks * n_dot_h.powf(shininess)
        } else {
            ZERO_VECTOR
        };

        // Update colour with diffuse and specular components
        colour += light_colour.component_mul(&((diffuse + specular) * falloff));
    }

    // Clamp colour values to [0, 255] and convert to u8
    let r = nalgebra::clamp(colour.x * 255.0, 0.0, 255.0) as u8;
    let g = nalgebra::clamp(colour.y * 255.0, 0.0, 255.0) as u8;
    let b = nalgebra::clamp(colour.z * 255.0, 0.0, 255.0) as u8;

    Vector3::new(r, g, b)
}
