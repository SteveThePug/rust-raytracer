use crate::{
    camera::Camera,
    light::Light,
    primitive::*,
    scene::{Node, Scene},
    state::{INIT_FILE, SAVE_FILE},
    EPSILON,
};
use imgui::*;
use nalgebra::{Point3, Vector3};
use pixels::{wgpu, PixelsContext};
use rhai::Engine;
use std::time::Instant;

const BUFFER_PROPORTION_INIT: f32 = 0.2;
const BUFFER_PROPORTION_MIN: f32 = 0.1;
const BUFFER_PROPORTION_MAX: f32 = 1.0;

const RAYS_INIT: i32 = 7000;
const RAYS_MIN: i32 = 100;
const RAYS_MAX: i32 = 30000;

const CAMERA_MIN_FOV: f32 = 10.0;
const CAMERA_MAX_FOV: f32 = 160.0;
const CAMERA_INIT: f32 = 5.0;

/// Manages all state required for rendering Dear ImGui over `Pixels`test.
pub enum GuiEvent {
    BufferResize(f32, f32),
    CameraUpdate(Camera, f32),
    SceneLoad(Scene),
    SaveImage(String),
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
    engine: Engine,
    scene: Scene,

    pub ray_num: i32,

    buffer_proportion: f32,

    camera_eye: [f32; 3],
    camera_target: [f32; 3],
    camera_up: [f32; 3],
    camera_fov: f32,

    image_filename: String,
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
        let font_size = (16.0 * hidpi_factor) as f32;
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
            engine: init_engine(),
            scene: Scene::empty(),

            ray_num: RAYS_INIT,
            buffer_proportion: BUFFER_PROPORTION_INIT,

            camera_eye: [CAMERA_INIT, CAMERA_INIT, CAMERA_INIT],
            camera_target: Vector3::zeros().into(),
            camera_up: Vector3::y().into(),
            camera_fov: 110.0,

            image_filename: String::from(SAVE_FILE),
        }
    }

    /// Prepare Dear ImGui.
    pub fn prepare(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), winit::error::ExternalError> {
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
        // let mut about_open = false;
        // ui.main_menu_bar(|| {
        //     ui.menu("Help", || {
        //         about_open = ui.menu_item("About...");
        //     });
        // });

        //Raytracing options -------------------------------------------
        if CollapsingHeader::new("Raytracer").build(ui) {
            // Numbers of rays to render
            ui.slider("# Rays: ", RAYS_MIN, RAYS_MAX, &mut self.ray_num);
            // Proportion of the window the buffer occupies
            ui.slider(
                "% Buffer: ",
                BUFFER_PROPORTION_MIN,
                BUFFER_PROPORTION_MAX,
                &mut self.buffer_proportion,
            );
            // Fov of the buffer
            ui.slider("fov", CAMERA_MIN_FOV, CAMERA_MAX_FOV, &mut self.camera_fov);
            // Apply stored changes
            if ui.button("Apply") {
                self.event = Some(GuiEvent::BufferResize(
                    self.buffer_proportion,
                    self.camera_fov,
                ));
            };
        }
        // CAMERA OPTIONS ----------------------------------------
        if CollapsingHeader::new("Camera").build(ui) {
            // Eye, target and up vector inputs
            ui.text("Camera options:");
            ui.input_float3("Eye", &mut self.camera_eye).build();
            ui.input_float3("Target", &mut self.camera_target).build();
            ui.input_float3("Up", &mut self.camera_up).build();
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
                );
                self.event = Some(GuiEvent::CameraUpdate(camera, self.camera_fov));
            }
        }
        // SCRIPTING --------------------------------------------
        if CollapsingHeader::new("Scripting").build(ui) {
            // Import file into multiline script
            ui.input_text("Scene file", &mut self.script_filename)
                .build();
            if ui.button("Import from File") {
                match std::fs::read_to_string(&mut self.script_filename) {
                    Ok(script) => {
                        self.script = script;
                    }
                    Err(e) => println!("{}", e),
                }
            }
            ui.same_line();
            // Load scene from multiline script using engine
            if ui.button("Load scene") {
                match self.engine.eval(&self.script) {
                    Ok(scene) => {
                        self.scene = scene;
                        self.event = Some(GuiEvent::SceneLoad(self.scene.clone()));
                    }
                    Err(e) => println!("{e}"),
                }
            }
            ui.same_line();
            // Save script to file
            if ui.button("Save script") {
                match std::fs::write(&self.script_filename, &self.script) {
                    Ok(_) => println!("Script saved successfully"),
                    Err(e) => println!("{}", e),
                }
            }
            // Multiline script
            ui.input_text_multiline("##", &mut self.script, [900., 300.])
                .build();
        }
        // IMAGE --------------------------------------------
        if CollapsingHeader::new("Image").build(ui) {
            // Image filename
            ui.input_text("Image file", &mut self.image_filename)
                .build();
            // Save image to file
            if ui.button("Save Image") {
                self.event = Some(GuiEvent::SaveImage(self.image_filename.clone()));
            }
        }
        // SCENE --------------------------------------------
        if CollapsingHeader::new("Scene").build(ui) {
            if ui.button("Update Scene") {
                for node in &mut self.scene.nodes {
                    node.compute();
                }
                self.event = Some(GuiEvent::SceneLoad(self.scene.clone()));
            }
            // Edit transformation of nodes
            if let Some(_t) = ui.tree_node("Nodes") {
                for node in &mut self.scene.nodes {
                    ui.text("node");
                    ui.slider_config("Translation", -10.0, 10.0)
                        .build_array(&mut node.translation);
                    ui.slider_config("Rotation", -180.0, 180.0)
                        .build_array(&mut node.rotation);
                    ui.slider_config("Scale", -10.0, 10.0)
                        .build_array(&mut node.scale);
                }
            }
            //Edit color, position and falloff of lights
            if let Some(_t) = ui.tree_node("Lights") {
                for light in &mut self.scene.lights {
                    ui.slider_config("Colour", 0.0, 1.0)
                        .build_array(light.colour.as_mut_slice());
                    ui.slider_config("Position", -10.0, 10.0)
                        .build_array(light.position.coords.as_mut_slice());
                    ui.slider_config("Falloff", 0.0, f32::MAX)
                        .build_array(light.falloff.as_mut_slice());
                }
            }
            //Use different cameras in the scene
            if let Some(_t) = ui.tree_node("Cameras") {
                for camera in &self.scene.cameras {
                    if ui.button("Use camera") {
                        GuiEvent::CameraUpdate(camera.clone(), self.camera_fov);
                    }
                }
            }
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

pub fn init_engine() -> Engine {
    let mut engine = Engine::new();

    engine
        .register_type::<Vector3<f64>>()
        .register_fn("V", Vector3::<f64>::new);
    engine
        .register_type::<Point3<f64>>()
        .register_fn("P", Point3::<f64>::new);
    engine
        .register_type::<Camera>()
        .register_fn("Camera", Camera::new);
    engine
        .register_type::<Scene>()
        .register_fn("Scene", Scene::empty)
        .register_fn("addNode", Scene::add_node)
        .register_fn("addLight", Scene::add_light)
        .register_fn("addCamera", Scene::add_camera)
        .register_fn("addMaterial", Scene::add_material);

    engine
        .register_type::<Node>()
        .register_fn("Node", Node::new)
        .register_fn("translate", Node::translate)
        .register_fn("rotate", Node::rotate)
        .register_fn("scale", Node::scale)
        .register_fn("child", Node::child);
    engine
        .register_type::<Light>()
        .register_fn("Light", Light::new)
        .register_fn("Ambient", Light::ambient);
    engine
        .register_type::<Material>()
        .register_fn("Material", Material::new)
        .register_fn("MaterialRed", Material::red)
        .register_fn("MaterialBlue", Material::blue)
        .register_fn("MaterialGreen", Material::green)
        .register_fn("MaterialMagenta", Material::magenta)
        .register_fn("MaterialTurquoise", Material::turquoise);
    engine
        .register_type::<Sphere>()
        .register_fn("Sphere", Sphere::new)
        .register_fn("SphereUnit", Sphere::unit);
    engine
        .register_type::<Cube>()
        .register_fn("Cube", Cube::new)
        .register_fn("CubeUnit", Cube::unit);
    engine
        .register_type::<Cone>()
        .register_fn("Cone", Cone::new)
        .register_fn("ConeUnit", Cone::unit);
    engine
        .register_type::<Cylinder>()
        .register_fn("Cylinder", Cylinder::new);
    engine
        .register_type::<Circle>()
        .register_fn("Circle", Circle::new)
        .register_fn("CircleUnit", Circle::unit);
    engine
        .register_type::<Rectangle>()
        .register_fn("Rectangle", Rectangle::new)
        .register_fn("RectangleUnit", Rectangle::unit);
    engine
        .register_type::<SteinerSurface>()
        .register_fn("Steiner", SteinerSurface::new);
    engine
        .register_type::<Torus>()
        .register_fn("Torus", Torus::new);
    engine
        .register_type::<Mesh>()
        .register_fn("Mesh", Mesh::from_file);
    engine
        .register_type::<Gnonom>()
        .register_fn("Gnonom", Gnonom::new);
    engine
}
