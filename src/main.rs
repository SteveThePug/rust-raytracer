#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module

//Cameras

use crate::camera::Camera;
use crate::gui::Gui;
use crate::light::Light;
use crate::primitive::*;
use crate::scene::Scene;

use error_iter::ErrorIter as _;
use log::error;
use nalgebra::{Point3, Vector3};
use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod camera;
mod gui;
mod light;
mod primitive;
mod ray;
mod raytracer;
mod scene;

const START_WIDTH: u32 = 640;
const START_HEIGHT: u32 = 480;
const BOX_SIZE: i16 = 64;

const EPSILON: f32 = 1e-7;
const INFINITY: f32 = 1e7;

struct State {
    scene: Scene,
    width: u32,
    height: u32,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(START_WIDTH as f64, START_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels + Dear ImGui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(START_WIDTH, START_HEIGHT, surface_texture)?
    };
    let mut state = State::new(START_WIDTH, START_HEIGHT);

    // Set up Dear ImGui
    let mut gui = Gui::new(&window, &pixels);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // Draw the world
            state.draw(pixels.frame_mut());
            // Prepare Dear ImGui
            gui.prepare(&window).expect("gui.prepare() failed");
            // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render the world texture
                context.scaling_renderer.render(encoder, render_target);
                // Render Dear ImGui
                gui.render(&window, encoder, render_target, context)?;
                Ok(())
            });
            // Basic error handling
            if let Err(err) = render_result {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        gui.handle_event(&window, &event);
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                if size.width > 0 && size.height > 0 {
                    // Resize the surface texture
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    // Resize the world
                    state.resize(size.width, size.height);
                    if let Err(err) = pixels.resize_buffer(size.width, size.height) {
                        log_error("pixels.resize_buffer", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
            }

            // Update internal state and request a redraw
            state.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl State {
    /// Create a new `World` instance that can draw a moving box.
    fn new(width: u32, height: u32) -> Self {
        let scene = Scene::empty();
        Self {
            width,
            height,
            scene,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {}

    /// Resize the world
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;

            //Create our scene
            let eye = Point3::new(0.0, 0.0, -1.0);
            let target = Point3::new(0.0, 0.0, 0.0);
            let up = Vector3::new(0.0, 1.0, 0.0);
            let arc_camera = Arc::new(Camera::new(
                eye,
                target,
                up,
                90.0,
                (self.width / self.height) as f32,
            ));
            let cameras: Vec<Arc<Camera>> = vec![arc_camera.clone()];

            let arc_material = Arc::new(Material::magenta());
            let arc_cone = Arc::new(Cone::unit(arc_material));
            let primitives: Vec<Arc<dyn Primitive>> = vec![arc_cone]
                .into_iter()
                .map(|arc| arc as Arc<dyn Primitive>)
                .collect();

            let light: Arc<Light>;
            let light = Arc::new(Light::white());
            let lights = vec![light];

            let ambient_light = Arc::new(Vector3::new(1.0, 1.0, 1.0));

            let scene = Scene::new(primitives, lights, cameras, ambient_light);

            let rays = arc_camera.as_ref().cast_rays(self.width, self.height);

            let colours = raytracer::shade_rays(&scene, &rays, self.width, self.height);
            println!("{}", colours.len());
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

            let rgba = [0x48, 0xb2, 0xe8, 0xff];

            pixel.copy_from_slice(&rgba);
        }
    }
}
