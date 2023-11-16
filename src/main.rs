#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module

//Cameras

use crate::{camera::Camera, gui::Gui, light::Light, primitive::*, ray::Ray, scene::Scene};
use log::error;

use error_iter::ErrorIter as _;
use nalgebra::{Point3, Vector3};
use pixels::{Error, Pixels, SurfaceTexture};
use std::env;
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

const START_WIDTH: i32 = 600;
const START_HEIGHT: i32 = 555;
const BOX_SIZE: i16 = 64;

const EPSILON: f32 = 1e-7;
const INFINITY: f32 = 1e7;

struct State {
    scene: Scene,
    camera: Camera,
    rays: Vec<Ray>,
    index: usize,
    width: i32,
    height: i32,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    //Window
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
    //Pixel surface
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(START_WIDTH as u32, START_HEIGHT as u32, surface_texture)?
    };
    //Camera
    let eye = Point3::new(0.0, 0.0, 3.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let arc_camera = Arc::new(Camera::new(
        eye,
        target,
        up,
        180.0,
        (START_WIDTH as f32 / START_HEIGHT as f32) as f32,
    ));
    let camera = Camera::new(
        eye,
        target,
        up,
        180.0,
        (START_WIDTH as f32 / START_HEIGHT as f32) as f32,
    );
    let cameras: Vec<Arc<Camera>> = vec![arc_camera.clone()];
    //Primitive
    let arc_material = Arc::new(Material::magenta());
    let mut primitives: Vec<Arc<dyn Primitive>> = Vec::new();
    let arc_sphere = Arc::new(Sphere::unit(arc_material.clone()));
    let arc_cone = Arc::new(Cone::unit(arc_material.clone()));
    primitives.push(arc_sphere.clone());
    primitives.push(arc_cone.clone());
    //Lights
    let light: Arc<Light>;
    let light = Arc::new(Light::white());
    let lights = vec![light];
    let ambient_light = Arc::new(Vector3::new(1.0, 1.0, 0.0));
    //State
    let scene = Scene::new(primitives, lights, cameras, ambient_light);
    let mut state = State::new(START_WIDTH, START_HEIGHT, scene, camera);
    // Set up Dear ImGui
    let mut gui = Gui::new(&window, &pixels);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            for i in 0..gui.num_rays {
                state.draw(pixels.frame_mut());
            }
            // Draw the world
            // Prepare Dear ImGui
            gui.prepare(&window).expect("gui.prepare() failed");
            // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render the world texture
                context.scaling_renderer.render(encoder, render_target);
                // Render Dear ImGui
                gui.render(&window, encoder, render_target, context)?;
                // *control_flow = ControlFlow::Exit;
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
            if input.key_pressed(VirtualKeyCode::A) {}
            // Resize the window
            if let Some(size) = input.window_resized() {
                if size.width > 0 && size.height > 0 {
                    // Resize the surface texture
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    state.resize(size.width as i32, size.height as i32);
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
    fn new(width: i32, height: i32, scene: Scene, camera: Camera) -> Self {
        let index = 0;
        let rays = camera.cast_rays(width, height);
        Self {
            width,
            height,
            index,
            rays,
            scene,
            camera,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {}

    /// Resize the world
    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        let ray = &self.rays[self.index];
        let colour = raytracer::shade_ray(&self.scene, &ray);
        let pixel = &mut frame[self.index * 4..(self.index + 1) * 4]
            .copy_from_slice(&[colour.x, colour.y, colour.z, 255]);
        self.index += 1;
    }

    fn draw_all(&mut self, frame: &mut [u8]) {
        let rays = self.camera.cast_rays(self.width, self.height);
        let colours = raytracer::shade_rays(&self.scene, &rays, self.width, self.height);
        for (i, colour) in colours.iter().enumerate() {
            let colour = colours[i];
            // pixel[0] = colour.x;
            // pixel[1] = colour.y;
            // pixel[2] = colour.z;
            // pixel[3] = 255;
        }
    }
}
