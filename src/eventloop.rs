use crate::state::State;
use core::iter;
use nalgebra::{Point3, Vector3};
use std::sync::Arc;
use std::time::Instant;

use pollster::block_on;
use wgpu::util::DeviceExt;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::light::Light;
use crate::primitive::Material;
use crate::{camera::Camera, primitive::*, raytracer, scene::Scene};

pub async fn run() {
    env_logger::init();
    // Set up window and GPU
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title(&format!("imgui-wgpu"));
    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => {
                            *control_flow = ControlFlow::Exit
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
