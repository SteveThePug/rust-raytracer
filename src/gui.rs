use nalgebra::Point3;
use pixels::{wgpu, PixelsContext};
use std::time::Instant;

const BUFFER_PROPORTION_INIT: f32 = 0.8;
const BUFFER_PROPORTION_MIN: f32 = 0.5;
const BUFFER_PROPORTION_MAX: f32 = 0.9;

const RAYS_INIT: i32 = 9000;
const RAYS_MIN: i32 = 100;
const RAYS_MAX: i32 = 10000;

const CAMERA_MIN: f32 = -10.0;
const CAMERA_MAX: f32 = 10.0;
const CAMERA_INIT: f32 = 5.0;

/// Manages all state required for rendering Dear ImGui over `Pixels`.
pub(crate) struct Gui {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    last_cursor: Option<imgui::MouseCursor>,
    about_open: bool,

    pub ray_num: i32,

    pub buffer_proportion: f32,
    pub buffer_resize: bool,

    pub camera_eye: Point3<f32>,
    pub camera_reposition: bool,
}

impl Gui {
    /// Create Dear ImGui.
    pub(crate) fn new(window: &winit::window::Window, pixels: &pixels::Pixels) -> Self {
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
        let font_size = (13.0 * hidpi_factor) as f32;
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
            about_open: true,
            ray_num: RAYS_INIT,
            buffer_proportion: BUFFER_PROPORTION_INIT,
            buffer_resize: false,
            camera_eye: Point3::new(CAMERA_INIT, CAMERA_INIT, CAMERA_INIT),
            camera_reposition: false,
        }
    }

    /// Prepare Dear ImGui.
    pub(crate) fn prepare(
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
    pub(crate) fn render(
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

        // Draw windows and GUI elements here
        let mut about_open = false;
        ui.main_menu_bar(|| {
            ui.menu("Help", || {
                about_open = ui.menu_item("About...");
            });
        });
        ui.slider("# Rays: ", RAYS_MIN, RAYS_MAX, &mut self.ray_num);

        ui.slider(
            "% Buffer: ",
            BUFFER_PROPORTION_MIN,
            BUFFER_PROPORTION_MAX,
            &mut self.buffer_proportion,
        );
        let mut buffer_resize = false;
        if ui.button("Change Buffer") {
            buffer_resize = true
        };
        self.buffer_resize = buffer_resize;

        let mut camera_reposition = false;
        ui.text("Vector3 Input:");
        // Create three input fields for x, y, and z components
        ui.slider("X", CAMERA_MIN, CAMERA_MAX, &mut self.camera_eye.coords[0]);
        ui.slider("Y", CAMERA_MIN, CAMERA_MAX, &mut self.camera_eye.coords[1]);
        ui.slider("Z", CAMERA_MIN, CAMERA_MAX, &mut self.camera_eye.coords[2]);
        // Check if any component of the Vector3 has changed
        if ui.button("Apply") {
            println!("Camera changed: {:?}", self.camera_eye);
            self.camera_eye = Point3::from(self.camera_eye);
            camera_reposition = true;
        }
        self.camera_reposition = camera_reposition;

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
    pub(crate) fn handle_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::Event<()>,
    ) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }
}
