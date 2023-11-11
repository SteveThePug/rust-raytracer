use std::borrow::Cow;
use std::fs;
use std::iter;

use wgpu::ShaderModuleDescriptor;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};

pub struct Display {
    //Surface we will draw two
    surface: wgpu::Surface,
    // Description of a surface
    config: wgpu::SurfaceConfiguration,
    //Device we will he using to render
    device: wgpu::Device,
    // Command query for a divice
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: winit::window::Window,
    // Handle to a rendering pipeline
    render_pipeline: wgpu::RenderPipeline,
    clear_color: wgpu::Color,
}

impl Display {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window, shader_filename: String) -> Self {
        //Self explaining, the window size
        let size = window.inner_size();
        // Backends to handle our gpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        // Surface needs to be alive as long as window is created
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        // Handles our graphics card, gets backend the adapter uses
        // Compatible surface fields tells wgpu whats a adapter that can supply a surface
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        // Use the adapter to create a device and queue
        // Includes limitations on the graphics card
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        // Some surfaces have different capabilities, get the capabilities
        let surface_caps = surface.get_capabilities(&adapter);
        // Find how surface textures will be stored on the GPU
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        //Load a shader into our device and get a module handler
        let shader_contents =
            fs::read_to_string(shader_filename).expect("Failed to read shader file");

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader.wgsl"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shader_contents)),
        });

        //Create pipeline layout using device
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        //Create final render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[],           // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        let clear_color = wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            clear_color,
        }
    }
}
impl Display {
    pub fn render(
        &mut self,
        render_function: fn(
            &wgpu::Device,
            &wgpu::Queue,
            &wgpu::SurfaceTexture,
            &wgpu::RenderPipeline,
        ) -> Result<(), wgpu::SurfaceError>,
    ) -> Result<(), wgpu::SurfaceError> {
        let texture = &self.surface.get_current_texture().expect("All good");
        render_function(&self.device, &self.queue, &texture, &self.render_pipeline)
    }
}
impl Display {
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn update_surface(&self) {
        self.resize(self.get_size())
    }
}
