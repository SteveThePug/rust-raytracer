#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module

use crate::primitive::*;
use crate::{camera::Camera, gui::Gui, light::Light, ray::Ray, scene::Scene};
use log::error;

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{env, thread, thread::JoinHandle};

use error_iter::ErrorIter as _;
use nalgebra::{Point3, Vector3};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

mod camera;
mod gui;
mod light;
mod primitive;
mod ray;
mod raytracer;
mod scene;

const START_WIDTH: i32 = 800;
const START_HEIGHT: i32 = 800;
const BOX_SIZE: i16 = 64;
const COLOUR_CLEAR: [u8; 4] = [0x22, 0x22, 0x11, 0xff];

const EPSILON: f32 = 1e-6;
const INFINITY: f32 = f32::MAX;
const EPSILON_VECTOR: Vector3<f32> = Vector3::new(EPSILON, EPSILON, EPSILON);
const INFINITY_VECTOR: Vector3<f32> = Vector3::new(INFINITY, INFINITY, INFINITY);

struct State {
    scene: Arc<Scene>,
    window: Window,
    pixels: Arc<Mutex<Pixels>>,
    gui: Gui,

    index: usize,

    camera: Camera,
    rays: Arc<Vec<Ray>>,
}

impl State {
    /// Create a new `World` instance that can draw a moving box.
    fn new(window: Window, scene: Scene, camera: Camera) -> Self {
        let window_size = window.inner_size();
        let pixels = {
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(
                window_size.width as u32,
                window_size.height as u32,
                surface_texture,
            )
            .unwrap()
        };
        let gui = Gui::new(&window, &pixels);
        let rays = camera.cast_rays(window_size.width, window_size.height);
        Self {
            scene: Arc::new(scene),
            window,
            pixels: Arc::new(Mutex::new(pixels)),
            gui,

            camera,
            index: 0,
            rays: Arc::new(rays),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) -> bool {
        if self.gui.buffer_resize {
            let pixels = &self.pixels;
            let size = self.window.inner_size();
            let width_new = (size.width as f32 * self.gui.buffer_proportion) as u32;
            let height_new = (size.height as f32 * self.gui.buffer_proportion) as u32;
            self.clear();
            let mut pixels = self.pixels.lock().unwrap();
            if let Err(err) = pixels.resize_buffer(width_new, height_new) {
                log_error("pixels.resize_surface", err);
                return false;
            }
            self.index = 0;
            self.rays = Arc::new(self.camera.cast_rays(width_new, height_new));
        }
        true
    }

    /// Resize the world
    fn resize(&mut self, size: &PhysicalSize<u32>) -> bool {
        println!("RESIZING!");
        let gui = &self.gui;
        let mut pixels = self.pixels.lock().unwrap();
        if let Err(err) = pixels.resize_surface(size.width, size.height) {
            log_error("pixels.resize_surface", err);
            return false;
        }
        true
    }

    fn keyboard_input(&mut self, key: &KeyboardInput) {
        println!("KEYBOARD INPUT");
        match key.virtual_keycode {
            Some(key) => match key {
                VirtualKeyCode::A => {}
                _ => {}
            },
            None => {}
        }
    }
    fn mouse_input(&mut self, button: &MouseButton) {
        println!("MOUSE INPUT");
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self) {
        // We want to multithread this function
        //let mut threads = vec![];
        // threads.push(thread::spawn({
        //     let pixels = self.pixels.clone();
        //     move || {
        //         // let colour = {
        //         //     let ray = &self.rays[i];
        //         //     raytracer::shade_ray(&self.scene, &ray)
        //         // };
        //
        //         // let rgba = match colour {
        //         //     Some(colour) => [colour.x, colour.y, colour.z, 255],
        //         //     None => COLOUR_CLEAR,
        //         // };
        //         let mut pixels = pixels.lock().unwrap();
        //         let frame = pixels.frame_mut().chunks_exact_mut(4).nth(i).unwrap();
        //         frame.copy_from_slice(&[200, 100, 100, 255]);
        //     }
        // }));

        for i in 0..self.gui.ray_num {
            let i = self.index as usize;
            let ray_num = self.gui.ray_num;
            let pixels = self.pixels.clone();
            let colour = {
                let ray = &self.rays[i];
                raytracer::shade_ray(&self.scene, &ray)
            };

            let rgba = match colour {
                Some(colour) => [colour.x, colour.y, colour.z, 255],
                None => COLOUR_CLEAR,
            };
            let mut pixels = self.pixels.lock().unwrap();
            let frame = pixels.frame_mut();
            frame[i * 4..(i + 1) * 4].copy_from_slice(&rgba);
            self.index = self.index + 1;
        }
    }

    fn clear(&mut self) {
        let mut pixels = self.pixels.lock().unwrap();
        let frame = pixels.frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = [0x00, 0x00, 0x00, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }

    fn render(&mut self) -> bool {
        self.update(); //Update state
        self.draw(); //Draw to pixels
        let pixels = self.pixels.lock().unwrap();
        self.gui
            .prepare(&self.window)
            .expect("gui.prepare() failed"); //Prepare imgui
        let render_result = pixels.render_with(|encoder, render_target, context| {
            context.scaling_renderer.render(encoder, render_target); // Render pixels
            self.gui
                .render(&self.window, encoder, render_target, context)?;
            Ok(())
        });
        if let Err(err) = render_result {
            log_error("pixels.render", err);
            return false;
        }
        true
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    env::set_var("RUST_BACKTRACE", "1");
    //Window
    let event_loop = EventLoop::new();
    //SCENE
    //Camera
    let eye = Point3::new(10.0, 10.0, 10.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        eye,
        target,
        up,
        120.0,
        (START_WIDTH as f32 / START_HEIGHT as f32) as f32,
    );
    // SETUP PRIMITIVES
    let magenta = Arc::new(Material::magenta());
    let blue = Arc::new(Material::blue());
    let turquoise = Arc::new(Material::turquoise());
    // let sphere = Arc::new(Sphere::unit(magenta.clone()));
    // primitives.push(sphere.clone());
    // let cone = Arc::new(Cone::new(0.25, 1.0, -0.5, turquoise.clone()));
    // primitives.push(cone.clone());
    let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    let cube = Box::new(Cube::unit(blue.clone()));
    primitives.push(cube);
    //Lights
    let light_pos = Point3::new(10.0, 12.0, 10.0);
    let light_colour = Vector3::new(1.0, 0.0, 1.0);
    let light_falloff = [1.0, 0.00, 0.00];
    let light = Light::new(light_colour, light_pos, light_falloff);

    let ambient_light = Vector3::new(0.0, 0.0, 0.2);

    let scene = Scene::new(primitives, vec![light], vec![camera.clone()], ambient_light);
    //State
    let window = {
        let size = LogicalSize::new(START_WIDTH, START_HEIGHT);
        WindowBuilder::new()
            .with_title("Hello Pixels + Dear ImGui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut state = State::new(window, scene, camera);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        state.gui.handle_event(&state.window, &event); //Let gui handle its events
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::Resized(size) => {
                    state.resize(&size);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    state.keyboard_input(&input);
                }
                WindowEvent::MouseInput { button, .. } => {
                    state.mouse_input(&button);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                state.render();
            }
            _ => {}
        }
        state.window.request_redraw(); //Redraw window
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
