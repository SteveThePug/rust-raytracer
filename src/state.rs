#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module

use crate::{camera::Camera, gui::Gui, light::Light, ray::Ray, scene::Scene};
use crate::{gui::GuiEvent, log_error};
use crate::{primitive::*, raytracer};

use std::error::Error;

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{env, thread, thread::JoinHandle};

use error_iter::ErrorIter as _;
use nalgebra::{Point3, Vector3};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

const START_WIDTH: i32 = 800;
const START_HEIGHT: i32 = 800;
const BOX_SIZE: i16 = 64;
const COLOUR_CLEAR: [u8; 4] = [0x22, 0x22, 0x11, 0xff];

pub fn run() {
    let event_loop = EventLoop::new();
    // Window
    let window = {
        let size = LogicalSize::new(START_WIDTH, START_HEIGHT);
        WindowBuilder::new()
            .with_title("Hello Pixels + Dear ImGui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let window_size = window.inner_size();
    //Display Surface
    let pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(
            window_size.width as u32,
            window_size.height as u32,
            surface_texture,
        )
        .unwrap()
    };
    //Gui
    let gui = Gui::new(&window, &pixels);

    //State
    let mut state = State::new(window, pixels, gui);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        state.gui.handle_event(&state.window, &event); //Let gui handle its events
        state.update().expect("Could not update");
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    state.resize(&size).expect("Could not resize");
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
                state.render().expect("Failed to render");
            }
            _ => {
                state.window.request_redraw();
            }
        }
    });
}

pub struct State {
    scene: Scene,
    rays: Vec<Ray>,

    window: Window,
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
        // Event Loop
        //Initalise empty scene for rendering
        let scene = Scene::empty();
        let window_size = window.inner_size();
        let rays = scene
            .camera
            .cast_rays(window_size.width, window_size.height);

        Self {
            scene,
            rays,
            window,
            pixels: Arc::new(Mutex::new(pixels)),
            gui,
            index: 0,
        }
    }

    /// Create a new `World` instance that can draw a moving box.

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let gui_event = self.gui.event.take();
        match gui_event {
            Some(event) => match event {
                GuiEvent::BufferResize | GuiEvent::CameraRelocate => {
                    let pixels = &self.pixels;
                    let size = self.window.inner_size();
                    let width_new = (size.width as f32 * self.gui.buffer_proportion) as u32;
                    let height_new = (size.height as f32 * self.gui.buffer_proportion) as u32;
                    self.clear();
                    let mut pixels = self.pixels.lock().unwrap();
                    pixels
                        .resize_buffer(width_new, height_new)
                        .expect("Resize Error");
                    self.scene.camera.set_position(self.gui.camera_eye);
                    self.rays = self.scene.camera.cast_rays(width_new, height_new);
                }
                GuiEvent::SceneLoad(filename) => {
                    match self.update_scene_from_file(&filename) {
                        Err(e) => {
                            println!()
                        }
                        Ok(()) => {
                            println!("Loaded file: {filename}")
                        }
                    }

                    self.clear();
                    println!("Reading {}", filename);
                }
            },
            None => {}
        }
        self.gui.event = None;
        Ok(())
    }

    /// Resize the world
    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<(), Box<dyn Error>> {
        println!("RESIZING!");
        let gui = &self.gui;
        let mut pixels = self.pixels.lock().unwrap();
        if let Err(err) = pixels.resize_surface(size.width, size.height) {
            log_error("pixels.resize_surface", err);
            return Ok(());
        }
        Ok(())
    }

    fn keyboard_input(&mut self, key: &KeyboardInput) {
        match key.virtual_keycode {
            Some(key) => match key {
                VirtualKeyCode::A => {}
                _ => {}
            },
            None => {}
        }
    }
    fn mouse_input(&mut self, button: &MouseButton) {}

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self) {
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
            self.index = (self.index + 1) % (frame.len() / 4);
        }
    }

    fn clear(&mut self) {
        self.index = 0;
        let mut pixels = self.pixels.lock().unwrap();
        let frame = pixels.frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = [0x00, 0x00, 0x00, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.update()?; //Update state
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
            return Ok(());
        }
        Ok(())
    }
}
