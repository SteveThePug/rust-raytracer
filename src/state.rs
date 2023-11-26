//Use linear algebra module

use crate::camera::Camera;
use crate::ray::Ray;
use crate::{gui::Gui, scene::Scene};
use crate::{gui::GuiEvent, log_error};
use std::path::Path;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::error::Error;

use std::sync::{Arc, Mutex};

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const START_WIDTH: i32 = 1200;
const START_HEIGHT: i32 = 1200;
const COLOUR_CLEAR: [u8; 4] = [0x22, 0x00, 0x11, 0xff];

pub const INIT_FILE: &str = "scene.rhai";
pub const SAVE_FILE: &str = "img.png";

pub struct State {
    scene: Scene,
    camera: Camera,
    window: Window,

    buffer_width: u32,
    buffer_height: u32,

    pixels: Arc<Mutex<Pixels>>,
    gui: Gui,

    rays: Vec<Ray>,
    ray_queue: Vec<usize>,
}

impl State {
    pub fn new(window: Window, pixels: Pixels, gui: Gui) -> Self {
        let scene = Scene::empty();
        let window_size = window.inner_size();
        let camera = Camera::unit();
        let rays = Vec::new();

        Self {
            scene,
            camera,
            window,
            buffer_width: window_size.width as u32,
            buffer_height: window_size.height as u32,
            pixels: Arc::new(Mutex::new(pixels)),
            gui,
            rays,
            ray_queue: Vec::new(),
        }
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(event) = self.gui.event.take() {
            match event {
                GuiEvent::BufferResize(proportion, fov) => {
                    self.resize_buffer(proportion, fov as f64)?
                }
                GuiEvent::CameraUpdate(camera) => {
                    self.camera = camera;
                    self.clear()?;
                    self.reset_queue();
                }
                GuiEvent::SceneLoad(scene) => {
                    self.scene = scene;
                    self.reset_queue();
                }
                GuiEvent::SaveImage(filename) => {
                    let pixels = self.pixels.lock().unwrap();
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

    fn resize_buffer(&mut self, proportion: f32, fovy: f64) -> Result<(), Box<dyn Error>> {
        let size = self.window.inner_size();

        self.buffer_width = (size.width as f32 * proportion) as u32;
        self.buffer_height = (size.height as f32 * proportion) as u32;

        self.clear()?;
        self.reset_queue();

        self.rays = Ray::cast_rays(fovy, self.buffer_width, self.buffer_height);

        let mut pixels = self.pixels.lock().unwrap();
        pixels.resize_buffer(self.buffer_width, self.buffer_height)?;
        pixels.resize_surface(size.width, size.height)?;
        Ok(())
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<(), Box<dyn Error>> {
        let mut pixels = self.pixels.lock().unwrap();
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
        for _ in 0..self.gui.ray_num {
            //Get random index from queue
            let index = self.ray_queue.pop().unwrap();
            //Shade colour for selected ray
            let colour = &self.rays[index].shade_ray(&self.scene);
            //Assign colour to frame
            let rgba = colour.map_or(COLOUR_CLEAR, |colour| [colour.x, colour.y, colour.z, 255]);
            let mut pixels = self.pixels.lock().unwrap();
            let frame = pixels.frame_mut();
            frame[index * 4..(index + 1) * 4].copy_from_slice(&rgba);

            if self.ray_queue.is_empty() {
                break;
            };
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        let mut pixels = self.pixels.lock().unwrap();
        let frame = pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }
        Ok(())
    }

    fn reset_queue(&mut self) {
        let size = self.buffer_height as usize * self.buffer_width as usize;
        let mut ray_queue: Vec<usize> = (0..size).collect();
        ray_queue.shuffle(&mut thread_rng());
        self.ray_queue = ray_queue;
        self.scene.compute(&self.camera.view, &self.camera.inv_view);
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.update()?; //Update state
        if !self.ray_queue.is_empty() {
            self.draw()?;
        }
        let pixels = self.pixels.lock().unwrap();
        self.gui
            .prepare(&self.window)
            .expect("gui.prepare() failed"); //Prepare imgui
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
    state.resize_buffer(1.0, 90.0)?;

    event_loop.run(move |event, _, control_flow| {
        state.gui.handle_event(&state.window, &event);

        if let Err(_e) = state.update() {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::WindowEvent { event, .. } => {
                if let Err(_e) = handle_window_event(event, control_flow, &mut state) {
                    *control_flow = ControlFlow::Exit;
                }
            }

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
        .build(event_loop)
        .unwrap()
}

fn create_pixels(window: &Window) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
    Pixels::new(1, 1, surface_texture).unwrap()
}

fn handle_window_event(
    event: WindowEvent,
    control_flow: &mut ControlFlow,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(size) => state.resize(&size)?,
        WindowEvent::KeyboardInput { input, .. } => state.keyboard_input(&input),
        WindowEvent::MouseInput { button, .. } => state.mouse_input(&button),
        _ => {}
    };
    Ok(())
}
