//Use linear algebra module

use crate::bvh::BVH;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::{gui::Gui, scene::Scene};
use crate::{gui::GuiEvent, log_error};
use std::path::Path;
use std::thread;

use nalgebra::Vector3;
use rand::seq::SliceRandom;
use rand::{random, thread_rng};

use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const START_WIDTH: i32 = 1200;
const START_HEIGHT: i32 = 700;

pub const INIT_FILE: &str = "rhai/scene.rhai";
pub const SAVE_FILE: &str = "img.png";

#[derive(Clone)]
pub struct RaytracingOption {
    pub threads: u32,
    pub ray_samples: u32,
    pub ray_randomness: f64,
    pub clear_color: [u8; 4],
    pub pixel_clear: [u8; 4],
    pub pixels_per_pass: u32,
    pub buffer_proportion: f32,
    pub buffer_fov: f64,
    pub ray_depth: u8,
    pub diffuse_rays: u8,
    pub diffuse_coefficient: f32,
    pub bvh_active: bool,
}
impl RaytracingOption {
    pub fn default() -> RaytracingOption {
        RaytracingOption {
            threads: 12,
            ray_samples: 10,
            ray_randomness: 100.0,
            clear_color: [0x22, 0x00, 0x11, 0x55],
            pixel_clear: [0x55, 0x00, 0x22, 0x55],
            pixels_per_pass: 200,
            buffer_proportion: 1.0,
            buffer_fov: 110.0,
            ray_depth: 5,
            diffuse_rays: 5,
            diffuse_coefficient: 0.5,
            bvh_active: false,
        }
    }
}

pub struct State {
    scene: Arc<Scene>,
    bvh: Arc<Option<BVH>>,
    camera: Camera,
    window: Window,

    buffer_width: u32,
    buffer_height: u32,

    pixels: Arc<Mutex<Pixels>>,
    gui: Gui,

    rays: Arc<Vec<Ray>>,
    ray_queue: Arc<Mutex<Vec<usize>>>,
    raytracing_options: Arc<RaytracingOption>,
}

impl State {
    pub fn new(window: Window, pixels: Pixels, gui: Gui) -> Self {
        let scene = Arc::new(Scene::empty());
        let window_size = window.inner_size();
        let pixels = Arc::new(Mutex::new(pixels));
        let camera = Camera::unit();
        let rays = Arc::new(Vec::new());

        Self {
            scene,
            bvh: Arc::new(None),
            camera,
            window,
            buffer_width: window_size.width as u32,
            buffer_height: window_size.height as u32,
            pixels,
            gui,
            rays,
            ray_queue: Arc::new(Mutex::new(Vec::new())),
            raytracing_options: Arc::new(RaytracingOption::default()),
        }
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(event) = self.gui.event.take() {
            match event {
                GuiEvent::RaytracerOption(options) => {
                    self.raytracing_options = Arc::new(options);
                    match self.raytracing_options.bvh_active {
                        true => self.bvh = Arc::new(Some(BVH::build(&self.scene.nodes))),
                        false => self.bvh = Arc::new(None),
                    }
                    self.resize_buffer()?
                }
                GuiEvent::CameraUpdate(camera) => {
                    self.rays = Arc::new(Ray::cast_rays(
                        &camera.eye,
                        &camera.target,
                        &camera.up,
                        self.raytracing_options.buffer_fov,
                        self.buffer_width,
                        self.buffer_height,
                    ));
                    self.camera = camera;
                    self.clear_buffer()?;
                    self.reset_queue();
                }
                GuiEvent::SceneLoad(scene) => {
                    self.scene = Arc::new(scene);
                    self.clear_buffer()?;
                    self.reset_queue();
                }
                GuiEvent::SaveImage(filename) => {
                    let pixels = &self.pixels.as_ref().lock().unwrap();
                    let frame = pixels.frame();
                    image::save_buffer(
                        Path::new(&filename),
                        frame,
                        self.buffer_width,
                        self.buffer_height,
                        image::ColorType::Rgba8,
                    )?
                }
            }
        };
        Ok(())
    }

    fn resize_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        // Calculate new buffer dimensions based on proportion
        let size = self.window.inner_size();
        let proportion = &self.raytracing_options.buffer_proportion;
        let fovy = self.raytracing_options.buffer_fov;
        self.buffer_width = (size.width as f32 * proportion) as u32;
        self.buffer_height = (size.height as f32 * proportion) as u32;

        // Clear the buffer and reset the ray queue
        self.clear_buffer()?;
        self.reset_queue();

        // Recalculate rays with new buffer dimensions
        self.rays = Arc::new(Ray::cast_rays(
            &self.camera.eye,
            &self.camera.target,
            &self.camera.up,
            fovy,
            self.buffer_width,
            self.buffer_height,
        ));

        // Resize buffer and surface
        let pixels = &mut self.pixels.as_ref().lock().unwrap();
        pixels.resize_surface(size.width, size.height)?;
        pixels.resize_buffer(self.buffer_width, self.buffer_height)?;

        Ok(())
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<(), Box<dyn Error>> {
        let pixels = &mut self.pixels.as_ref().lock().unwrap();
        pixels.resize_surface(size.width, size.height)?;
        Ok(())
    }

    fn keyboard_input(&mut self, key: &KeyboardInput) {
        if let Some(VirtualKeyCode::A) = key.virtual_keycode {
            // Handle 'A' key event here
        }
    }

    fn mouse_input(&mut self, _button: &MouseButton) {
        // Handle mouse input here
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        //Draw ray_num in a block
        let randomness = self.raytracing_options.ray_randomness;
        let samples = self.raytracing_options.ray_samples;
        let samples_f32 = samples as f32;
        let mut handles = vec![];

        for _ in 0..self.raytracing_options.pixels_per_pass {
            for _ in 0..self.raytracing_options.threads {
                //Get random index from queue
                let queue = &mut self.ray_queue.clone();
                let index = match queue.lock().unwrap().pop() {
                    Some(index) => index,
                    None => break,
                };
                //Create a nre thread for this pixel
                let handle = thread::spawn({
                    let rays = self.rays.clone();
                    let scene = self.scene.clone();
                    let options = self.raytracing_options.clone();
                    let bvh = self.bvh.clone();
                    let rays = rays.clone();
                    let pixels_mutex = self.pixels.clone();
                    move || {
                        //Shade colour for selected ray
                        let mut colour: Vector3<f32> = Vector3::zeros();
                        for _ in 0..samples {
                            let ray = &rays[index];
                            let point = ray.a;
                            let dir = ray.b;
                            let rx = (random::<f64>() - 0.5) / randomness;
                            let ry = (random::<f64>() - 0.5) / randomness;
                            let rz = (random::<f64>() - 0.5) / randomness;
                            let nx = dir.x + rx;
                            let ny = dir.y + ry;
                            let nz = dir.z + rz;

                            let rand_ray = Ray::new(point, Vector3::new(nx, ny, nz));

                            if let Some(ray_colour) = rand_ray.shade_ray(&scene, 0, &options, &bvh)
                            {
                                colour += ray_colour;
                            }
                        }
                        colour = (colour / samples_f32) * 255.0;
                        let rgba = [colour.x as u8, colour.y as u8, colour.z as u8, 0xff];
                        let pixels = &mut pixels_mutex.lock().unwrap();
                        let frame = pixels.frame_mut();
                        frame[index * 4..(index + 1) * 4].copy_from_slice(&rgba);
                    }
                });
                handles.push(handle);
            }
            for handle in handles.drain(..) {
                handle.join().unwrap();
            }
        }
        Ok(())
    }

    fn clear_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        let pixels = &mut self.pixels.as_ref().lock().unwrap();
        let frame = pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&self.raytracing_options.pixel_clear);
        }
        Ok(())
    }

    fn reset_queue(&mut self) {
        match self.raytracing_options.bvh_active {
            true => self.bvh = Arc::new(Some(BVH::build(&self.scene.nodes))),
            false => self.bvh = Arc::new(None),
        }
        let size = self.buffer_height as usize * self.buffer_width as usize;
        let mut ray_queue: Vec<usize> = (0..size).collect();
        ray_queue.shuffle(&mut thread_rng());
        self.ray_queue = Arc::new(Mutex::new(ray_queue));
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        // Update state
        self.update()?;
        // Draw rays if we have remaining rays in queue
        match self.draw() {
            Err(e) => {
                println!("ERROR: {}", e);
            }
            _ => {}
        }
        // Render Gui
        self.gui
            .prepare(&self.window)
            .expect("gui.prepare() failed");
        // Try to render pixels
        let pixels = &mut self.pixels.as_ref().lock().unwrap();
        if let Err(e) = pixels.render_with(|encoder, render_target, context| {
            context.scaling_renderer.render(encoder, render_target); // Render pixels
            self.gui
                .render(&self.window, encoder, render_target, context)?;

            Ok(())
        }) {
            log_error("pixels.render", e);
        };

        Ok(())
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);
    let pixels = create_pixels(&window);
    let gui = Gui::new(&window, &pixels);

    let mut state = State::new(window, pixels, gui);
    state.resize_buffer()?;

    event_loop.run(move |event, _, control_flow| {
        state.gui.handle_event(&state.window, &event);

        if let Err(_e) = state.update() {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => state.resize(&size).expect("Window Resize Error"),
                WindowEvent::KeyboardInput { input, .. } => state.keyboard_input(&input),
                WindowEvent::MouseInput { button, .. } => state.mouse_input(&button),
                _ => {}
            },

            Event::RedrawRequested(_) => {
                if let Err(_e) = state.render() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => state.window.request_redraw(),
        }
    })
}

fn create_window(event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(START_WIDTH, START_HEIGHT);
    WindowBuilder::new()
        .with_title("Hello Pixels + Dear ImGui")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_transparent(true)
        .build(event_loop)
        .unwrap()
}

fn create_pixels(window: &Window) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
    Pixels::new(1, 1, surface_texture).unwrap()
}
