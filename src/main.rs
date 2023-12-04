use crate::state::run;
use error_iter::ErrorIter;

const EPSILON: f64 = 1e-8;
const INFINITY: f64 = 1e10;

use log::error;
use std::env;
use std::error::Error;

mod bvh;
mod camera;
mod gui;
mod light;
mod material;
mod node;
mod primitive;
mod ray;
mod scene;
mod state;

fn main() {
    env_logger::init();
    env::set_var("RUST_BACKTRACE", "1");
    //let args: Vec<String> = env::args().collect();
    if let Err(e) = run() {
        println!("Error at runtime: {}", e);
    };

    // if args.len() == 6 {
    //     let width: usize = args[1].parse().unwrap();
    //     let height: usize = args[2].parse().unwrap();
    //     let fovy = args[3].parse::<f64>().unwrap();
    //     let filename = &args[4];
    //     let savefile = &args[5];
    //     headless(
    //         width,
    //         height,
    //         fovy,
    //         filename.to_string(),
    //         savefile.to_string(),
    //     );
    // } else {
    //}
}

// fn headless(width: usize, height: usize, fovy: f64, filename: String, savefile: String) {
//     let options = Arc::new(RaytracingOption {
//         threads: 12,
//         ray_samples: 1,
//         ray_randomness: 100.0,
//         clear_color: [0x22, 0x00, 0x11, 0x55],
//         pixel_clear: [0x55, 0x00, 0x22, 0x55],
//         pixels_per_thread: 200,
//         buffer_proportion: 1.0,
//         buffer_fov: 110.0,
//         ray_depth: 5,
//         diffuse_rays: 3,
//         diffuse_coefficient: 0.8,
//         bvh_active: false,
//     });
//     //Read script from file
//     let script = match std::fs::read_to_string(&filename) {
//         Ok(in_script) => in_script,
//         Err(e) => {
//             println!("{}", e);
//             return;
//         }
//     };
//     //Evaluate scene in file
//     let engine = init_engine();
//     let scene: Arc<Scene> = match engine.eval(&script) {
//         Ok(in_scene) => Arc::new(in_scene),
//         Err(e) => {
//             println!("{e}");
//             return;
//         }
//     };
//     //Set the camera
//     let mut camera = Camera::unit();
//     for (_, in_camera) in &scene.cameras {
//         camera = in_camera.clone();
//     }
//     //Cast the rays
//     let rays = Arc::new(Ray::cast_rays(
//         &camera.eye,
//         &camera.target,
//         &camera.up,
//         fovy,
//         width as u32,
//         height as u32,
//     ));
//     //Enable bounding volume heirarchy
//     let bvh;
//     match options.bvh_active {
//         true => bvh = Arc::new(Some(BVH::build(&scene.nodes))),
//         false => bvh = Arc::new(None),
//     }
//     //Create our frame and indexer
//     let size = width * height;
//     let frame_mutex = Arc::new(Mutex::new(vec![0; size * 4]));
//     //Multithreading
//     let mut handles = vec![];

//     for index in 0..size {
//         for _ in 0..options.threads {
//             //Get random index from queue
//             //Create a nre thread for this pixel
//             let handle = thread::spawn({
//                 let rays = rays.clone();
//                 let scene = scene.clone();
//                 let options = options.clone();
//                 let bvh = bvh.clone();
//                 let rays = rays.clone();
//                 let frame_mutex = frame_mutex.clone();
//                 move || {
//                     //Shade colour for selected ray
//                     let mut colour: Vector3<f32> = Vector3::zeros();
//                     //Get the ray we want to make
//                     let shot_ray = &rays[index];
//                     //Send out ray_samples rays
//                     for _ in 0..options.ray_samples {
//                         let point = shot_ray.a;
//                         let dir = shot_ray.b;
//                         //Generate a random ray
//                         let rx = (random::<f64>() - 0.5) / options.ray_randomness;
//                         let ry = (random::<f64>() - 0.5) / options.ray_randomness;
//                         let rz = (random::<f64>() - 0.5) / options.ray_randomness;
//                         let nx = dir.x + rx;
//                         let ny = dir.y + ry;
//                         let nz = dir.z + rz;
//                         let rand_ray = Ray::new(point, Vector3::new(nx, ny, nz));

//                         if let Some(ray_colour) = rand_ray.shade_ray(&scene, 0, &options, &bvh) {
//                             colour += ray_colour;
//                         }
//                     }
//                     colour = (colour / options.ray_samples as f32) * 255.0;
//                     let rgba = [colour.x as u8, colour.y as u8, colour.z as u8, 0xff];
//                     {
//                         let frame = &mut frame_mutex.lock().unwrap();
//                         frame[index * 4..(index + 1) * 4].copy_from_slice(&rgba);
//                     }
//                 }
//             });
//             handles.push(handle);
//         }
//         for handle in handles.drain(..) {
//             handle.join().unwrap();
//         }
//     }
//     use std::path::Path;
//     image::save_buffer(
//         Path::new(&savefile),
//         &frame_mutex.lock().unwrap(),
//         width as u32,
//         height as u32,
//         image::ColorType::Rgba8,
//     )
//     .unwrap();
// }

fn log_error<E: Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
