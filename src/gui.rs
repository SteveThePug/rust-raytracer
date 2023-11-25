use crate::{camera::Camera, scene::Scene, state::INIT_FILE, UP_VECTOR_F32, ZERO_VECTOR_F32};
use imgui::*;
use nalgebra::{Point3, Vector3};
use pixels::{wgpu, PixelsContext};
use std::time::Instant;

const BUFFER_PROPORTION_INIT: f32 = 1.0;
const BUFFER_PROPORTION_MIN: f32 = 0.5;
const BUFFER_PROPORTION_MAX: f32 = 1.0;

const RAYS_INIT: i32 = 9000;
const RAYS_MIN: i32 = 100;
const RAYS_MAX: i32 = 10000;

const CAMERA_MIN_FOV: f32 = 10.0;
const CAMERA_MAX_FOV: f32 = 160.0;
const CAMERA_INIT: f32 = 5.0;

/// Manages all state required for rendering Dear ImGui over `Pixels`test.
pub enum GuiEvent {
    BufferResize(f32),
    CameraUpdate(Camera),
    SceneLoad(Scene),
}

pub struct Gui {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    last_cursor: Option<imgui::MouseCursor>,

    pub event: Option<GuiEvent>,

    script_filename: String,
    script: String,
    scene: Scene,

    pub ray_num: i32,

    buffer_proportion: f32,

    camera_eye: [f32; 3],
    camera_target: [f32; 3],
    camera_up: [f32; 3],
    camera_fov: f32,
}

impl Gui {
    /// Create Dear ImGui.
    pub fn new(window: &winit::window::Window, pixels: &pixels::Pixels) -> Self {
        // Create Dear ImGui context
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        // Initialize winit platform support
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );

        // Configure Dear ImGui fonts
        let hidpi_factor = window.scale_factor();
        let font_size = (11.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        // Create Dear ImGui WGPU renderer
        let device = pixels.device();
        let queue = pixels.queue();
        let config = imgui_wgpu::RendererConfig {
            texture_format: pixels.render_texture_format(),
            ..Default::default()
        };
        let renderer = imgui_wgpu::Renderer::new(&mut imgui, device, queue, config);

        // Return GUI context
        Self {
            imgui,
            platform,
            renderer,
            last_frame: Instant::now(),
            last_cursor: None,
            event: None,

            script_filename: String::from(INIT_FILE),
            script: String::new(),
            scene: Scene::empty(),

            ray_num: RAYS_INIT,
            buffer_proportion: BUFFER_PROPORTION_INIT,

            camera_eye: [CAMERA_INIT, CAMERA_INIT, CAMERA_INIT],
            camera_target: ZERO_VECTOR_F32.into(),
            camera_up: UP_VECTOR_F32.into(),
            camera_fov: 110.0,
        }
    }

    /// Prepare Dear ImGuBi.
    pub fn prepare(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), winit::error::ExternalError> {
        // Prepare Dear ImGui
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
        self.platform.prepare_frame(self.imgui.io_mut(), window)
    }

    /// Render Dear ImGui.
    pub fn render(
        &mut self,
        window: &winit::window::Window,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) -> imgui_wgpu::RendererResult<()> {
        // Start a new Dear ImGui frame and update the cursor
        let ui = self.imgui.new_frame();

        let mouse_cursor = ui.mouse_cursor();
        if self.last_cursor != mouse_cursor {
            self.last_cursor = mouse_cursor;
            self.platform.prepare_render(ui, window);
        }

        //Top Menu Bar
        let mut about_open = false;
        ui.main_menu_bar(|| {
            ui.menu("Help", || {
                about_open = ui.menu_item("About...");
            });
        });

        //Raytracing options
        if CollapsingHeader::new("Raytracer").build(ui) {
            //Ray Renderer
            ui.slider("# Rays: ", RAYS_MIN, RAYS_MAX, &mut self.ray_num);
            //Buffer Options
            ui.slider(
                "% Buffer: ",
                BUFFER_PROPORTION_MIN,
                BUFFER_PROPORTION_MAX,
                &mut self.buffer_proportion,
            );
            //Apply changes
            if ui.button("Apply") {
                self.event = Some(GuiEvent::BufferResize(self.buffer_proportion));
            };
        }
        //Camera options
        if CollapsingHeader::new("Camera").build(ui) {
            ui.text("Camera options:");
            ui.input_float3("Eye", &mut self.camera_eye).build();
            ui.input_float3("Target", &mut self.camera_target).build();
            ui.input_float3("Up", &mut self.camera_up).build();
            ui.slider("fov", CAMERA_MIN_FOV, CAMERA_MAX_FOV, &mut self.camera_fov);
            // Create three input fields for x, y, and z components
            if ui.button("Apply Camera") {
                println!("Camera changed: {:?}", self.camera_eye);
                let (eye, target, up) = (&self.camera_eye, &self.camera_target, &self.camera_up);
                let (ex, ey, ez) = (eye[0] as f64, eye[1] as f64, eye[2] as f64);
                let (tx, ty, tz) = (target[0] as f64, target[1] as f64, target[2] as f64);
                let (ux, uy, uz) = (up[0] as f64, up[1] as f64, up[2] as f64);

                let camera = Camera::new(
                    Point3::new(ex, ey, ez),
                    Point3::new(tx, ty, tz),
                    Vector3::new(ux, uy, uz),
                    1,
                    1,
                    self.camera_fov as f64,
                );
                self.event = Some(GuiEvent::CameraUpdate(camera));
            }
        }
        //Scripting
        if CollapsingHeader::new("Scripting").build(ui) {
            //Import from file (We just want to replace the contents of self.script)
            ui.input_text("Scene file", &mut self.script_filename)
                .build();
            if ui.button("Import from File") {
                match std::fs::read_to_string(&self.script_filename) {
                    Ok(script) => self.script = script,
                    Err(e) => println!("{e}"),
                }
            }
            if ui.button("Apply script") {
                match Scene::from_rhai(&self.script) {
                    Ok(scene) => {
                        self.scene = scene;
                        self.event = Some(GuiEvent::SceneLoad(self.scene.clone()));
                    }
                    Err(e) => println!("{e}"),
                }
            }
            //Script block
            ui.input_text_multiline("script", &mut self.script, [500., 900.])
                .build();
        }

        // Render Dear ImGui with WGPU
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("imgui"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.renderer.render(
            self.imgui.render(),
            &context.queue,
            &context.device,
            &mut rpass,
        )
    }

    /// Handle any outstanding events.
    pub fn handle_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::Event<()>,
    ) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }
}
