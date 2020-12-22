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

fn trivial() -> ocl::Result<()> {
    // let src = r#"
    //     __kernel 
    //     void add(__global float2* buffer, float scalar) {
    //         buffer[get_global_id(0)] += scalar;
    //     }
    // "#;

    let src = read_file("./src/kernels/Sphere.ocl");

    let pro_que = ProQue::builder()
        .src(src)
        .dims(1 << 20)
        .build()?;

    let spheres = pro_que.create_buffer::<Sphere>()?;
    
    let kernel = pro_que.kernel_builder("compute")
        .arg(&spheres)
        .arg(10.0f32)
        .build()?;

    unsafe { kernel.enq()?; }

    // let mut vec = vec![Sphere::default(); spheres.len()];
    let mut vec = Vec::new();
    spheres.read(&mut vec).enq()?;

    println!("The value at 50 is now '{:?}'!", vec[50].pos);
    unsafe { kernel.enq()?; }
    spheres.read(&mut vec).enq()?;

    println!("The value at 50 is now '{:?}'!", vec[50].pos);
    Ok(())
}


fn test_scene(){
    let mut scene = Scene::new(720,480).unwrap();
    scene.compute().unwrap();
    let screen = scene.get_screen();
    for i in 172800..172805 {
        print!("{}-",screen[i])
    }
}

fn main() {
    
    // trivial().unwrap();
    // test_scene();
    window(&mut Scene::new(720,480).unwrap());

    // println!("{}", ( Instant::now() -first).as_secs_f64());
    // trivial_explained().unwrap();
    // trivial_exploded().unwrap();
    // trivial_cored().unwrap();
}