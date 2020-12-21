use std::time::Instant;

use minifb::*;

use crate::graphics::Scene;




pub fn window(scene : &mut Scene) {

    let mut window =
        Window::new(
            "Noise Test - Press ESC to exit",
            scene.width as usize,
            scene.height as usize,
            WindowOptions {
                resize: true,
                scale_mode: ScaleMode::Center,
                ..WindowOptions::default()
            },
        )
        .expect("Unable to open Window");
    let mut elapsed;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let first = Instant::now();
        // update buffer
        // println!("compute color");
        // compute_color(scene).unwrap();

        let convert = Instant::now();
        let mut buf = vec![0, scene.width * scene.height];
        scene.screen.read(&mut buf).enq().unwrap();
        println!("Time converting : {}", (Instant::now() - convert).as_secs_f64());
        // ###########
        window
            .update_with_buffer(&buf, scene.width as usize, scene.height as usize)
            .unwrap();
        let last = Instant::now();
        elapsed = (last-first).as_secs_f64();
        println!("{:.1} fps", 1.0/elapsed);
    }

}