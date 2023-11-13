use pixels::{Error, Pixels, SurfaceTexture};

use std::time::Instant;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() -> Result<(), Error> {
    // Create an event loop and window using winit
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Create a Pixels instance for drawing
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(800, 600, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        //We want to change the size of the pixel display
                        pixels
                            .resize_surface(physical_size.width, physical_size.height)
                            .expect("Could not resize");
                        pixels
                            .resize_buffer(physical_size.width, physical_size.height)
                            .expect("Could not resize");
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        //We want to change the size of the pixel display
                        // new_inner_size is &mut so w have to dereference it twice
                        pixels
                            .resize_surface(new_inner_size.width, new_inner_size.height)
                            .expect("Could not resize!");
                        pixels
                            .resize_buffer(new_inner_size.width, new_inner_size.height)
                            .expect("Could not resize");
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        println!("Key input: {}", input.scancode);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                // Draw a pixel at every coordinate
                for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                    let r = (i as u8).wrapping_mul(50);
                    let g = 43;
                    let b = 3;
                    let a = 255;
                    pixel.copy_from_slice(&[r, g, b, a]);
                }
                // Render the frame
                if pixels.render().is_err() {
                    eprintln!("Failed to render frame");
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
