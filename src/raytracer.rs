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
        let to_light = Unit::new_normalize(to_light);
        // Point to camera
        let to_camera = Unit::new_normalize(-incidence.into_inner());
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
