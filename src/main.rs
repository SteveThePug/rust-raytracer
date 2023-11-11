#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use our ray module
mod ray;
use ray::Ray;
//Use our display
pub mod display;
use display::Display;
//Use linear algebra module
use nalgebra::Vector4;
//Use modules for wgpu
use std::iter;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};

pub struct Program {
    display: Display,
}

impl Program {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let display = Display::new(window, "shader.wgsl".to_string()).await;
        Program { display }
    }
impl Program {}
    async fn start_event_loop(mut self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !self.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.display.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.display.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                self.update();
                match self.display.render(Self::render) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        self.display.update_surface();
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        });
    }

    fn input(&self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { .. } => true,
            WindowEvent::KeyboardInput { input, .. } => {
                match input.virtual_keycode.expect("Not a keycode") {
                    VirtualKeyCode::Space => true,
                    _ => false,
                }
            }
            _ => true,
        }
    }

    fn update(&mut self) {}

    fn render(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        output: &wgpu::SurfaceTexture,
        render_pipeline: &wgpu::RenderPipeline,
    ) -> Result<(), wgpu::SurfaceError> {
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            b: 1.0,
                            g: 1.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(render_pipeline);
            render_pass.draw(0..3, 0..1); // 3.
        }

        // submit will accept anything that implements IntoIter
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

//MAIN ---------------------------------------------------------------------------------
#[tokio::main]
async fn main() {
    let program = Program::new();
    program.start_event_loop().await;
}
