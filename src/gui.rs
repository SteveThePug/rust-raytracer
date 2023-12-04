use crate::{
    camera::Camera,
    light::Light,
    material::*,
    node::*,
    primitive::*,
    scene::*,
    state::{RaytracingOption, INIT_FILE, SAVE_FILE},
};
use imgui::*;
use nalgebra::{Point3, Vector3};
use pixels::{wgpu, PixelsContext};
use rhai::Engine;
use std::time::Instant;

//BUFFER CONSTANTS
const BUFFER_PROPORTION_MIN: f32 = 0.1;
const BUFFER_PROPORTION_MAX: f32 = 1.0;

//RAY CONSTANTS
const MIN_THREADS: u32 = 1;
const MAX_THREADS: u32 = 12;
const RAYS_MIN: u32 = 100;
const RAYS_MAX: u32 = 10000;
const MIN_DEPTH: u8 = 1;
const MAX_DEPTH: u8 = 10;
const MIN_SAMPLES: u32 = 1;
const MAX_SAMPLES: u32 = 10;
const MIN_RANDOM: f64 = 100.0;
const MAX_RANDOM: f64 = 1000.0;

//DIFFUSE CONSTANTS
const MIN_DIFFUSE_RAYS: u8 = 1;
const MAX_DIFFUSE_RAYS: u8 = 10;
const MIN_DIFFUSE_COEFFICIENT: f32 = 0.0;
const MAX_DIFFUSE_COEFFICIENT: f32 = 1.0;

//MATERIAL CONSTANTS
const MIN_D: f32 = 0.0;
const MIN_S: f32 = 0.0;
const MIN_SHINE: f32 = 0.0;
const MAX_D: f32 = 1.0;
const MAX_S: f32 = 1.0;
const MAX_SHINE: f32 = 50.0;

//TRANSFORMATION CONSTANTS
const MIN_COLOUR: f32 = 0.0;
const MIN_FALLOFF: f32 = 0.0;
const MIN_SCALE: f64 = 0.0;
//const MIN_POSITION: f64 = -10.0;
const MIN_ROTATION: f64 = -180.0;
const MIN_TRANSLATE: f64 = -10.0;
//--
const MAX_COLOUR: f32 = 1.0;
const MAX_FALLOFF: f32 = 1.0;
const MAX_SCALE: f64 = 3.0;
//const MAX_POSITION: f64 = 10.0;
const MAX_ROTATION: f64 = 180.0;
const MAX_TRANSLATE: f64 = 10.0;

// CAMERA CONSTANTS
const MIN_FOV: f64 = 10.0;
const MAX_FOV: f64 = 160.0;
//const CAMERA_INIT: f32 = 5.0;

/// Manages all state required for rendering Dear ImGui over `Pixels`test.
pub enum GuiEvent {
    RaytracerOption(RaytracingOption),
    CameraUpdate(Camera),
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

    raytracing_option: RaytracingOption,

    camera: Camera,

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
        let mut gui = Self {
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

            raytracing_option: RaytracingOption::default(),

            camera: Camera::unit(),

            image_filename: String::from(SAVE_FILE),
        };

        // ------------ TESTING CODE (LOAD SCENE ON START) -----------------
        match std::fs::read_to_string(&mut gui.script_filename) {
            Ok(script) => {
                gui.script = script;
            }
            Err(e) => println!("{}", e),
        }
        match gui.engine.eval(&gui.script) {
            Ok(scene) => {
                gui.scene = scene;
                gui.event = Some(GuiEvent::SceneLoad(gui.scene.clone()));
            }
            Err(e) => println!("{e}"),
        }
        // ------------ TESTING CODE (LOAD SCENE ON START) -----------------
        gui
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
            ui.slider(
                "Threads",
                MIN_THREADS,
                MAX_THREADS,
                &mut self.raytracing_option.threads,
            );
            // Numbers of rays to render per pass
            ui.slider(
                "Rays Per Pass",
                RAYS_MIN,
                RAYS_MAX,
                &mut self.raytracing_option.pixels_per_thread,
            );
            // Proportion of the window the buffer occupies
            ui.slider(
                "% Buffer: ",
                BUFFER_PROPORTION_MIN,
                BUFFER_PROPORTION_MAX,
                &mut self.raytracing_option.buffer_proportion,
            );
            //Clear colour for scene
            ui.slider_config("Clear Colour", 0, 255)
                .build_array(&mut self.raytracing_option.clear_color);
            //Clear colour if no intersect
            ui.slider_config("Pixel Clear Colour", 0, 255)
                .build_array(&mut self.raytracing_option.pixel_clear);
            //Ray depth slider
            ui.slider(
                "Ray Depth",
                MIN_DEPTH,
                MAX_DEPTH,
                &mut self.raytracing_option.ray_depth,
            );
            //Ray samples slider
            ui.slider(
                "Ray Samples",
                MIN_SAMPLES,
                MAX_SAMPLES,
                &mut self.raytracing_option.ray_samples,
            );
            //Ray randomness
            ui.slider(
                "Ray Randomness",
                MIN_RANDOM,
                MAX_RANDOM,
                &mut self.raytracing_option.ray_randomness,
            );
            //Number of diffuse rays
            ui.slider(
                "Diffuse Rays",
                MIN_DIFFUSE_RAYS,
                MAX_DIFFUSE_RAYS,
                &mut self.raytracing_option.diffuse_rays,
            );
            //Diffuse Coefficient
            ui.slider(
                "Diffuse Coefficient",
                MIN_DIFFUSE_COEFFICIENT,
                MAX_DIFFUSE_COEFFICIENT,
                &mut self.raytracing_option.diffuse_coefficient,
            );
            // Fov of the buffer
            ui.slider(
                "fov",
                MIN_FOV,
                MAX_FOV,
                &mut self.raytracing_option.buffer_fov,
            );
            // Enable BVH
            ui.checkbox("Enable BVH", &mut self.raytracing_option.bvh_active);
            // Apply stored changes
            if ui.button("Apply") {
                self.event = Some(GuiEvent::RaytracerOption(self.raytracing_option.clone()));
            };
        }
        // CAMERA OPTIONS ----------------------------------------
        if CollapsingHeader::new("Camera").build(ui) {
            // Eye, target and up vector inputs
            ui.text("Camera options:");
            ui.slider_config("Eye", MIN_TRANSLATE, MAX_TRANSLATE)
                .build_array(self.camera.eye.coords.as_mut_slice());
            ui.slider_config("Target", MIN_TRANSLATE, MAX_TRANSLATE)
                .build_array(self.camera.target.coords.as_mut_slice());
            ui.slider_config("Up", 0.0, 1.0)
                .build_array(self.camera.up.as_mut_slice());
            if ui.button("Apply Camera") {
                println!("Camera changed");
                self.event = Some(GuiEvent::CameraUpdate(self.camera.clone()));
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
                for (_, node) in &mut self.scene.nodes {
                    node.compute();
                }
                self.event = Some(GuiEvent::SceneLoad(self.scene.clone()));
            }
            // Edit transformation of nodes
            if let Some(_t) = ui.tree_node("Nodes") {
                for (label, node) in &mut self.scene.nodes {
                    ui.checkbox(format!("##active{label}"), &mut node.active);
                    ui.same_line();
                    if let Some(_t) = ui.tree_node(label) {
                        ui.slider_config("Translation", MIN_TRANSLATE, MAX_TRANSLATE)
                            .build_array(&mut node.translation);
                        ui.slider_config("Rotation", MIN_ROTATION, MAX_ROTATION)
                            .build_array(&mut node.rotation);
                        ui.slider_config("Scale", MIN_SCALE, MAX_SCALE)
                            .build_array(&mut node.scale);
                    }
                }
            }
            // Edit materials
            if let Some(_t) = ui.tree_node("Materials") {
                for (label, material) in &mut self.scene.materials {
                    if let Some(_t) = ui.tree_node(label) {
                        ui.slider_config("ks", MIN_D, MAX_D)
                            .build_array(material.ks.as_mut_slice());
                        ui.slider_config("kd", MIN_S, MAX_S)
                            .build_array(material.kd.as_mut_slice());
                        ui.slider("shine", MIN_SHINE, MAX_SHINE, &mut material.shininess);
                    }
                }
            }
            //Edit color, position and falloff of lights
            if let Some(_t) = ui.tree_node("Lights") {
                for (label, light) in &mut self.scene.lights {
                    ui.checkbox(format!("##activelight{label}"), &mut light.active);
                    ui.same_line();
                    if let Some(_t) = ui.tree_node(label) {
                        ui.slider_config("Colour", MIN_COLOUR, MAX_COLOUR)
                            .build_array(light.colour.as_mut_slice());
                        ui.slider_config("Position", MIN_TRANSLATE, MAX_TRANSLATE)
                            .build_array(light.position.coords.as_mut_slice());
                        ui.slider_config("Falloff", MIN_FALLOFF, MAX_FALLOFF)
                            .build_array(light.falloff.as_mut_slice());
                    }
                }
            }
            //Use different cameras in the scene
            if let Some(_t) = ui.tree_node("Cameras") {
                for (label, camera) in &self.scene.cameras {
                    if ui.button(label) {
                        self.camera = camera.clone();
                        self.event = Some(GuiEvent::CameraUpdate(camera.clone()));
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
        .register_fn("child", Node::child)
        .register_fn("active", Node::set_active);
    engine
        .register_type::<Light>()
        .register_fn("Light", Light::new)
        .register_fn("Ambient", Light::ambient)
        .register_fn("active", Light::set_active);
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
        .register_type::<Triangle>()
        .register_fn("Triangle", Triangle::new)
        .register_fn("TriangleUnit", Triangle::unit);
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
        .register_type::<Cube>()
        .register_fn("Cube", Cube::new)
        .register_fn("CubeUnit", Cube::unit);
    engine
        .register_type::<Steiner>()
        .register_fn("Steiner", Steiner::new);
    engine
        .register_type::<Steiner2>()
        .register_fn("Steiner2", Steiner2::new);
    engine
        .register_type::<Roman>()
        .register_fn("Roman", Roman::new);
    engine
        .register_type::<CrossCap>()
        .register_fn("CrossCap", CrossCap::new);
    engine
        .register_type::<CrossCap2>()
        .register_fn("CrossCap2", CrossCap2::new);
    engine
        .register_type::<Torus>()
        .register_fn("Torus", Torus::new);
    engine
        .register_type::<Gnonom>()
        .register_fn("Gnonom", Gnonom::new);
    engine
        .register_type::<Mesh>()
        .register_fn("Mesh", Mesh::from_file);
    engine
        .register_type::<RectangleXY>()
        .register_fn("Rectange", RectangleXY::new)
        .register_fn("RectangleUnit", RectangleXY::unit);
    engine
}
