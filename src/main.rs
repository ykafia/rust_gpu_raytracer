use std::time::Instant;

use ocl::{Buffer, OclPrm, ProQue, prm::{Float2, Float3}};

// mod maths;
// use maths::*;

mod util;
use util::*;

mod graphics;
use graphics::*;
use window::window;

mod window;

fn trivial() {
    // let src = r#"
    //     __kernel 
    //     void add(__global float2* buffer, float scalar) {
    //         buffer[get_global_id(0)] += scalar;
    //     }
    // "#;

    let src = read_file("./src/kernels/Sphere.ocl");

    let pro_que = ProQue::builder()
        .src(src)
        .dims(50)
        .build().unwrap();

    let spheres = pro_que.create_buffer::<Sphere>().unwrap();
    let screen = pro_que.create_buffer::<u32>().unwrap();
    let mut sp = vec![Sphere::default();50];
    let mut sc = vec![0u32;50];

    spheres.write(&sp).enq().unwrap();
    screen.write(&sc).enq().unwrap();
    
    let kernel = pro_que.kernel_builder("compute")
        .arg(&spheres)
        .arg(&screen)
        .build().unwrap();

    unsafe { kernel.enq().unwrap(); }

    // let mut vec = vec![Sphere::default(); spheres.len()];
    // let mut vec = Vec::new();
    spheres.read(&mut sp).enq().unwrap();

    println!("The value at 50 is now '{:?}'!", sp[0].pos);
    unsafe { kernel.enq().unwrap(); }
    spheres.read(&mut sp).enq().unwrap();

    println!("The value at 50 is now '{:?}'!", sp[49].pos);
}


fn test_scene(){
    let mut scene = Scene::new(720,480);
    scene.compute().unwrap();
    let screen = scene.get_screen();
    for i in 172800..172805 {
        print!("{}-",screen[i])
    }
}

fn main() {
    println!("{}",std::mem::size_of::<Camera>());
    // trivial();
    // test_scene();
    window(&mut Scene::new(720,480));

    // println!("{}", ( Instant::now() -first).as_secs_f64());
    // trivial_explained().unwrap();
    // trivial_exploded().unwrap();
    // trivial_cored().unwrap();
}