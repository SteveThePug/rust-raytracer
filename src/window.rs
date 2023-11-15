use nalgebra::{Point3, Vector3};
use std::sync::Arc;
use std::time::Instant;

use wgpu::util::DeviceExt;
use winit::{
    error::EventLoopError,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::light::Light;
use crate::primitive::Material;
use crate::{camera::Camera, primitive::*, raytracer, scene::Scene};

pub fn run() -> Result<(), EventLoopError> {
    // Create an event loop and window using winit
    let event_loop = EventLoop::new().expect("Could not make event loop");
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let window_size = &window.inner_size();

    //Environment variables
    let mut height = window_size.height;
    let mut width = window_size.width;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(physical_size) => {
                        width = physical_size.width;
                        height = physical_size.height;
                        // draw_pixels(&mut pixels, 400, 400);
                    }
                    WindowEvent::ScaleFactorChanged {
                        inner_size_writer, ..
                    } => {}
                    WindowEvent::KeyboardInput { event, .. } => {
                        println!("{}", event.logical_key.to_text().unwrap());
                    }
                    WindowEvent::RedrawRequested => {}
                    _ => {}
                }
            }
            _ => {}
        }
    })
}

// fn clear_pixels(pixels: &mut Pixels) {
//     let frame = pixels.frame_mut();
//     for pixel in frame.chunks_exact_mut(4) {
//         pixel[0] = 0x00; // R
//         pixel[1] = 0x00; // G
//         pixel[2] = 0x00; // B
//         pixel[3] = 0xff; // A
//     }
//     if pixels.render().is_err() {
//         eprintln!("Failed to render frame");
//     }
// }
//
// fn draw_pixels(pixels: &mut Pixels, width: u32, height: u32) {
//     //Create our scene
//     let eye = Point3::new(0.0, 0.0, -1.0);
//     let target = Point3::new(0.0, 0.0, 0.0);
//     let up = Vector3::new(0.0, 1.0, 0.0);
//     let arc_camera = Arc::new(Camera::new(eye, target, up, 90.0, (width / height) as f32));
//     let cameras: Vec<Arc<Camera>> = vec![arc_camera.clone()];
//
//     let arc_material = Arc::new(Material::magenta());
//     let arc_cone = Arc::new(Cone::unit(arc_material));
//     let primitives: Vec<Arc<dyn Primitive>> = vec![arc_cone]
//         .into_iter()
//         .map(|arc| arc as Arc<dyn Primitive>)
//         .collect();
//
//     let light: Arc<Light>;
//     let light = Arc::new(Light::white());
//     let lights = vec![light];
//
//     let ambient_light = Arc::new(Vector3::new(1.0, 1.0, 1.0));
//
//     let scene = Scene::new(primitives, lights, cameras, ambient_light);
//
//     let rays = arc_camera.as_ref().cast_rays(width, height);
//
//     let colours = raytracer::shade_rays(&scene, &rays, width, height);
//
//     print!("{}", colours.len());
//
//     let pixels = pixels.frame().chunks_exact_mut(4);
//     for (i, colour) in colours.iter().enumerate() {
//         let colour = colours[i];
//         let pixel = &mut pixels[i];
//         pixel[0] = colour.x;
//         pixel[1] = colour.y;
//         pixel[2] = colour.z;
//     }
//
//     // Render the frame
//     if pixels.render().is_err() {
//         eprintln!("Failed to render frame");
//     }
// }
// // Create a Pixels instance for drawing
// let mut pixels = {
//     let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//     Pixels::new(width, height, surface_texture).unwrap()
// };
