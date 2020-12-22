use std::time::Instant;

use minifb::*;

use crate::graphics::Scene;




pub fn window(scene : &mut Scene) {

    let mut window =
        Window::new(
            "CL-Rays",
            scene.width as usize,
            scene.height as usize,
            WindowOptions {
                resize: true,
                scale_mode: ScaleMode::Center,
                borderless: true,
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
        scene.compute().unwrap();
        // ###########
        let (height,width) = (scene.height as usize,scene.width as usize);
        window
            .update_with_buffer(&scene.get_screen(), width, height)
            .unwrap();
        let last = Instant::now();
        elapsed = (last-first).as_secs_f64();
        println!("{:.1} fps", 1.0/elapsed);
    }

}