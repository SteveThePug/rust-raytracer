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
    pub pixels_per_thread: u32,
    pub buffer_proportion: f32,
    pub buffer_fov: f64,
    pub ray_depth: u8,
    pub diffuse_rays: u8,
    pub diffuse_coefficient: f32,
    pub bvh_active: bool,
    pub shadows: bool,
    pub diffuse: bool,
    pub reflect: bool,
    pub specular: bool,
    pub falloff: bool,
}
impl RaytracingOption {
    pub fn default() -> RaytracingOption {
        RaytracingOption {
            threads: 12,
            ray_samples: 1,
            ray_randomness: 100.0,
            clear_color: [0x22, 0x00, 0x11, 0x55],
            pixel_clear: [0x55, 0x00, 0x22, 0x55],
            pixels_per_thread: 200,
            buffer_proportion: 1.0,
            buffer_fov: 110.0,
            ray_depth: 5,
            diffuse_rays: 3,
            diffuse_coefficient: 0.8,
            bvh_active: false,
            shadows: true,
            diffuse: true,
            reflect: true,
            specular: true,
            falloff: true,
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

    pixels: Pixels,
    gui: Gui,

    rays: Arc<Vec<Ray>>,
    ray_queue: Vec<usize>,
    raytracing_options: Arc<RaytracingOption>,
}

impl State {
    pub fn new(window: Window, pixels: Pixels, gui: Gui) -> Self {
        let scene = Arc::new(Scene::empty());
        let window_size = window.inner_size();
        let pixels = pixels;
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
            ray_queue: Vec::new(),
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
                    let frame = self.pixels.frame();
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
        self.pixels.resize_surface(size.width, size.height)?;
        self.pixels
            .resize_buffer(self.buffer_width, self.buffer_height)?;

        Ok(())
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<(), Box<dyn Error>> {
        self.pixels.resize_surface(size.width, size.height)?;
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

        let num_threads = self.raytracing_options.threads;
        let pixels_per_thread = self.raytracing_options.pixels_per_thread;

        let mut handles = vec![];

        for _ in 0..num_threads {
            //Get necessary variables to render
            let rays = self.rays.clone();
            let scene = self.scene.clone();
            let options = self.raytracing_options.clone();
            let bvh = self.bvh.clone();

            //Get the workload for a thread
            let mut load = vec![];
            for _ in 0..pixels_per_thread {
                match self.ray_queue.pop() {
                    Some(index) => load.push(index),
                    None => break,
                }
            }
            //The finished queue of the thread
            let mut finished = vec![];

            //Create a new thread for these pixels
            let handle = thread::spawn({
                move || {
                    for index in &load {
                        //Shade colour for selected index
                        let mut colour: Vector3<f32> = Vector3::zeros();
                        let ray = &rays[*index];
                        for _ in 0..samples {
                            //Generate a ray in a random direction
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
                        finished.push(rgba);
                    }
                    return (load, finished);
                }
            });
            handles.push(handle);
        }

        let mut all_results = vec![];

        for handle in handles.drain(..) {
            let (load, finished) = handle
                .join()
                .map_err(|e| format!("Thread panicked: {:?}", e))?;
            let thread_results: Vec<_> = load.into_iter().zip(finished.into_iter()).collect();
            all_results.extend(thread_results);
        }

        //Now we have two vectors will all the indicies and rgba values, we can upload them to the bufer

        let frame = self.pixels.frame_mut();
        for result in all_results {
            let index = result.0;
            let rgba = result.1;
            frame[index * 4..(index + 1) * 4].copy_from_slice(&rgba);
        }
        Ok(())
    }

    fn clear_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        let frame = self.pixels.frame_mut();
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
        self.ray_queue = ray_queue;
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
        if let Err(e) = self.pixels.render_with(|encoder, render_target, context| {
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
