use std::{io::{Write, stdout}, time::Instant};

use minifb::*;
use ocl::prm::Float4;

use crate::graphics::{Scene, Sphere};




pub fn window(scene : &mut Scene) {
    let (height,width) = (scene.height as usize,scene.width as usize);
    let mut window =
        Window::new(
            "CL-Rays",
            scene.width as usize,
            scene.height as usize,
            WindowOptions {
                resize: true,
                scale_mode: ScaleMode::AspectRatioStretch,
                borderless: false,
                title: true,
                scale: Scale::FitScreen,
                ..Default::default()
            },
        )
        .expect("Unable to open Window");
    let mut total = 0f64;
    let mut elapsed;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let first = Instant::now();
        // update buffer
        // println!("compute color");
        // scene.update_spheres().unwrap();
        scene.compute().unwrap();
        // ###########
        
        window
            .update_with_buffer(&scene.get_screen(), width, height)
            .unwrap();
        let last = Instant::now();
        elapsed = (last-first).as_secs_f64();
        // scene.camera.pos = Float4::new(0,0,-8,0);        
        scene.spheres_buf.read(&mut scene.spheres).enq().unwrap();
        // println!("float3 size : {}",&scene.get_screen()[0]);
        total += elapsed;
        scene.camera.pos = Float4::new((total.sin() * 3.0)  as f32,1.0,(total.cos() *3.0) as f32 -10.0,0.0);
        scene.camera.face_towards(scene.spheres[0].pos);
        if window.is_key_pressed(Key::T, KeyRepeat::No){
            scene.add_sphere().unwrap();
        }
        scene.update_spheres().unwrap();
        scene.update_kernel();
        
        // if total % 5.0 < 0.01 {scene.add_sphere().unwrap();}
        print!("{:.1} fps - Spheres number : {}\r", 1.0/elapsed, scene.spheres.len());
    }

}