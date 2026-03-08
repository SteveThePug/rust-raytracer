//Use linear algebra module

use crate::bvh::BVH;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::{gui::Gui, scene::Scene};
use crate::{gui::GuiEvent, log_error};
use std::collections::HashSet;
use std::path::Path;
use std::thread;

use nalgebra::Vector3;
use rand::seq::SliceRandom;
use rand::{random, thread_rng};

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{
    ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};
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
            ray_randomness: 700.0,
            clear_color: [0x22, 0x00, 0x11, 0x55],
            pixel_clear: [0x11, 0x00, 0x22, 0x55],
            pixels_per_thread: 100,
            buffer_proportion: 1.0,
            buffer_fov: 70.0,
            ray_depth: 1,
            diffuse_rays: 3,
            diffuse_coefficient: 0.1,
            bvh_active: false,
            shadows: true,
            diffuse: true,
            reflect: true,
            specular: true,
            falloff: true,
        }
    }
}

const CAMERA_MOVE_SPEED: f64 = 0.15;
const CAMERA_ORBIT_SPEED: f64 = 0.005;

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
    ray_queue: Arc<Mutex<Vec<usize>>>,
    raytracing_options: Arc<RaytracingOption>,

    result_rx: mpsc::Receiver<Vec<(usize, [u8; 4])>>,
    render_active: Arc<AtomicBool>,
    rendering: bool,

    keys_pressed: HashSet<VirtualKeyCode>,
    right_mouse_down: bool,
    last_mouse_pos: Option<(f64, f64)>,
    camera_dirty: bool,
}

impl State {
    pub fn new(window: Window, pixels: Pixels, gui: Gui) -> Self {
        let scene = Arc::new(Scene::empty());
        let window_size = window.inner_size();
        let pixels = pixels;
        let camera = Camera::unit();
        let rays = Arc::new(Vec::new());
        let (_tx, rx) = mpsc::channel();

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
            result_rx: rx,
            render_active: Arc::new(AtomicBool::new(false)),
            rendering: false,
            keys_pressed: HashSet::new(),
            right_mouse_down: false,
            last_mouse_pos: None,
            camera_dirty: false,
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
        if let Some(keycode) = key.virtual_keycode {
            match key.state {
                ElementState::Pressed => {
                    self.keys_pressed.insert(keycode);
                }
                ElementState::Released => {
                    self.keys_pressed.remove(&keycode);
                }
            }
        }
    }

    fn mouse_input(&mut self, button: &MouseButton, state: &ElementState) {
        if *button == MouseButton::Right {
            self.right_mouse_down = *state == ElementState::Pressed;
            if !self.right_mouse_down {
                self.last_mouse_pos = None;
            }
        }
    }

    fn cursor_moved(&mut self, x: f64, y: f64) {
        if self.right_mouse_down {
            if let Some((last_x, last_y)) = self.last_mouse_pos {
                let dx = x - last_x;
                let dy = y - last_y;
                self.camera.orbit(
                    -dx * CAMERA_ORBIT_SPEED,
                    -dy * CAMERA_ORBIT_SPEED,
                );
                self.camera_dirty = true;
            }
            self.last_mouse_pos = Some((x, y));
        }
    }

    fn process_camera_movement(&mut self) {
        let speed = CAMERA_MOVE_SPEED;

        if self.keys_pressed.contains(&VirtualKeyCode::W) {
            self.camera.move_forward(speed);
            self.camera_dirty = true;
        }
        if self.keys_pressed.contains(&VirtualKeyCode::S) {
            self.camera.move_forward(-speed);
            self.camera_dirty = true;
        }
        if self.keys_pressed.contains(&VirtualKeyCode::A) {
            self.camera.move_right(-speed);
            self.camera_dirty = true;
        }
        if self.keys_pressed.contains(&VirtualKeyCode::D) {
            self.camera.move_right(speed);
            self.camera_dirty = true;
        }
        if self.keys_pressed.contains(&VirtualKeyCode::Q) {
            self.camera.move_up(-speed);
            self.camera_dirty = true;
        }
        if self.keys_pressed.contains(&VirtualKeyCode::E) {
            self.camera.move_up(speed);
            self.camera_dirty = true;
        }

        if self.camera_dirty {
            self.camera_dirty = false;
            self.rays = Arc::new(Ray::cast_rays(
                &self.camera.eye,
                &self.camera.target,
                &self.camera.up,
                self.raytracing_options.buffer_fov,
                self.buffer_width,
                self.buffer_height,
            ));
            self.gui.update_camera(&self.camera);
            let _ = self.clear_buffer();
            self.reset_queue();
        }
    }

    fn draw(&mut self) {
        if !self.rendering {
            return;
        }

        // Drain completed results from background workers
        loop {
            match self.result_rx.try_recv() {
                Ok(results) => {
                    let frame = self.pixels.frame_mut();
                    for (index, rgba) in results {
                        frame[index * 4..(index + 1) * 4].copy_from_slice(&rgba);
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    // All worker threads have finished
                    self.rendering = false;
                    self.gui.stop_render_timer();
                    break;
                }
            }
        }
    }

    fn clear_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&self.raytracing_options.pixel_clear);
        }
        Ok(())
    }

    fn reset_queue(&mut self) {
        // Signal any existing workers to stop
        self.render_active.store(false, Ordering::Relaxed);

        match self.raytracing_options.bvh_active {
            true => self.bvh = Arc::new(Some(BVH::build(&self.scene.nodes))),
            false => self.bvh = Arc::new(None),
        }

        // Create new shuffled queue
        let size = self.buffer_height as usize * self.buffer_width as usize;
        let mut ray_queue: Vec<usize> = (0..size).collect();
        ray_queue.shuffle(&mut thread_rng());
        self.ray_queue = Arc::new(Mutex::new(ray_queue));

        // Create new channel and active flag
        let (tx, rx) = mpsc::channel();
        self.result_rx = rx;
        let render_active = Arc::new(AtomicBool::new(true));
        self.render_active = render_active.clone();
        self.rendering = true;

        // Spawn persistent worker threads
        let num_threads = self.raytracing_options.threads;
        let pixels_per_thread = self.raytracing_options.pixels_per_thread;

        for _ in 0..num_threads {
            let rays = self.rays.clone();
            let scene = self.scene.clone();
            let options = self.raytracing_options.clone();
            let bvh = self.bvh.clone();
            let queue = self.ray_queue.clone();
            let tx = tx.clone();
            let active = render_active.clone();

            thread::spawn(move || {
                let randomness = options.ray_randomness;
                let samples = options.ray_samples;
                let samples_f32 = samples as f32;

                loop {
                    if !active.load(Ordering::Relaxed) {
                        break;
                    }

                    // Pop a batch from the shared queue
                    let load: Vec<usize> = {
                        let mut q = queue.lock().unwrap();
                        let mut batch = Vec::with_capacity(pixels_per_thread as usize);
                        for _ in 0..pixels_per_thread {
                            match q.pop() {
                                Some(index) => batch.push(index),
                                None => break,
                            }
                        }
                        batch
                    };

                    if load.is_empty() {
                        break;
                    }

                    // Process the batch
                    let mut results = Vec::with_capacity(load.len());
                    for index in &load {
                        let mut colour: Vector3<f32> = Vector3::zeros();
                        let ray = &rays[*index];
                        for _ in 0..samples {
                            let point = ray.a;
                            let dir = ray.b;
                            let rx = (random::<f64>() - 0.5) / randomness;
                            let ry = (random::<f64>() - 0.5) / randomness;
                            let rz = (random::<f64>() - 0.5) / randomness;
                            let nx = dir.x + rx;
                            let ny = dir.y + ry;
                            let nz = dir.z + rz;

                            let rand_ray = Ray::new(point, Vector3::new(nx, ny, nz));

                            if let Some(ray_colour) =
                                rand_ray.shade_ray(&scene, 0, &options, &bvh)
                            {
                                colour += ray_colour;
                            }
                        }
                        colour = (colour / samples_f32) * 255.0;
                        let rgba = [colour.x as u8, colour.y as u8, colour.z as u8, 0xff];
                        results.push((*index, rgba));
                    }

                    // Send results back to main thread
                    if tx.send(results).is_err() {
                        break;
                    }
                }
            });
        }
        // Drop our copy of tx so the channel disconnects when all workers finish
        drop(tx);

        self.gui.start_render_timer();
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        // Update state
        self.update()?;
        // Collect completed rays from background workers
        self.draw();
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
                WindowEvent::MouseInput { button, state: elem_state, .. } => {
                    state.mouse_input(&button, &elem_state)
                }
                WindowEvent::CursorMoved { position, .. } => {
                    state.cursor_moved(position.x, position.y)
                }
                _ => {}
            },

            Event::RedrawRequested(_) => {
                state.process_camera_movement();
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
