//Use linear algebra module

use crate::raytracer::*;
use crate::{gui::Gui, ray::Ray, scene::Scene};
use crate::{gui::GuiEvent, log_error};

use std::error::Error;

use std::sync::{Arc, Mutex};

use pixels::{Pixels, SurfaceTexture, TextureError};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const START_WIDTH: i32 = 800;
const START_HEIGHT: i32 = 800;
const BOX_SIZE: i16 = 64;
const COLOUR_CLEAR: [u8; 4] = [0x22, 0x22, 0x11, 0xff];

pub fn run() {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);
    let pixels = create_pixels(&window);
    let gui = Gui::new(&window, &pixels);

    let mut state = State::new(window, pixels, gui);
    state.clear();

    event_loop.run(move |event, _, control_flow| {
        state.gui.handle_event(&state.window, &event);
        state.update().expect("Could not update");

        match event {
            Event::WindowEvent { event, .. } => {
                handle_window_event(event, control_flow, &mut state)
            }
            Event::RedrawRequested(_) => state.render().expect("Failed to render"),
            _ => state.window.request_redraw(),
        }
    });
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

fn handle_window_event(event: WindowEvent, control_flow: &mut ControlFlow, state: &mut State) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(size) => state.resize(&size).expect("Could not resize"),
        WindowEvent::KeyboardInput { input, .. } => state.keyboard_input(&input),
        WindowEvent::MouseInput { button, .. } => state.mouse_input(&button),
        _ => {}
    }
}

pub struct State {
    scene: Scene,
    rays: Vec<Ray>,
    window: Window,
    buffer_width: u32,
    buffer_height: u32,
    pixels: Arc<Mutex<Pixels>>,
    gui: Gui,
    index: usize,
}

impl State {
    pub fn update_scene_from_file(&mut self, filename: &String) -> Result<(), Box<dyn Error>> {
        self.scene = Scene::from_script(filename)?.into();
        let window_size = self.window.inner_size();
        self.rays = self
            .scene
            .camera
            .cast_rays(window_size.width, window_size.height);
        Ok(())
    }

    pub fn new(window: Window, pixels: Pixels, gui: Gui) -> Self {
        let scene = Scene::empty();
        let window_size = window.inner_size();
        let rays = scene
            .camera
            .cast_rays(window_size.width, window_size.height);

        Self {
            scene,
            rays,
            window,
            buffer_width: (window_size.width as f32 * gui.buffer_proportion) as u32,
            buffer_height: (window_size.height as f32 * gui.buffer_proportion) as u32,
            pixels: Arc::new(Mutex::new(pixels)),
            gui,
            index: 0,
        }
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(event) = self.gui.event.take() {
            match event {
                GuiEvent::BufferResize | GuiEvent::CameraRelocate => self.resize_buffer(),
                GuiEvent::SceneLoad(filename) => self.load_scene(&filename),
            }
        }
        Ok(())
    }

    fn resize_buffer(&mut self) {
        let size = self.window.inner_size();
        self.buffer_width = (size.width as f32 * self.gui.buffer_proportion) as u32;
        self.buffer_height = (size.height as f32 * self.gui.buffer_proportion) as u32;
        self.clear();
        let mut pixels = self.pixels.lock().unwrap();
        pixels
            .resize_buffer(self.buffer_width, self.buffer_height)
            .expect("Resize Error");
    }

    fn load_scene(&mut self, filename: &String) {
        println!("Reading {}", filename);
        match self.update_scene_from_file(filename) {
            Err(e) => println!("{}", e),
            Ok(()) => println!("Loaded file: {filename}"),
        }
        self.clear();
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<(), TextureError> {
        let mut pixels = self.pixels.lock().unwrap();
        pixels.resize_surface(size.width, size.height)
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
            let i = self.index as usize;
            let colour = self.rays[i].shade_ray(&self.scene);
            let rgba = colour.map_or(COLOUR_CLEAR, |colour| [colour.x, colour.y, colour.z, 255]);
            let mut pixels = self.pixels.lock().unwrap();
            let frame = pixels.frame_mut();
            frame[i * 4..(i + 1) * 4].copy_from_slice(&rgba);
            self.index = (self.index + 1) % (frame.len() / 4);
        }
        Ok(())
    }

    fn clear(&mut self) {
        self.index = 0;
        let mut pixels = self.pixels.lock().unwrap();
        let frame = pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }
        self.scene.camera.set_position(self.gui.camera_eye);
        self.rays = self
            .scene
            .camera
            .cast_rays(self.buffer_width, self.buffer_height);
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.update()?; //Update state
        self.draw()?; //Draw to pixels
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
        }
        Ok(())
    }
}
