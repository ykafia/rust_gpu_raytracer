use std::{io::{Write, stdout}, time::Instant};

use minifb::*;

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
        
        // scene.spheres_buf.read(&mut scene.spheres).enq().unwrap();
        println!("float3 size : {}",&scene.get_screen()[0]);
        print!("{:.1} fps\r", 1.0/elapsed);
    }

}