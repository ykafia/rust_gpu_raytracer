use std::time::Instant;

use ocl::{Buffer, OclPrm, ProQue, prm::{Float2, Float3}};

// mod maths;
// use maths::*;

mod util;
use util::*;

mod graphics;
use graphics::*;

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

    let spheres = pro_que.create_buffer::<Matrix3x3>()?;
    
    let kernel = pro_que.kernel_builder("compute")
        .arg(&spheres)
        .arg(10.0f32)
        .build()?;

    unsafe { kernel.enq()?; }

    let mut vec = vec![Matrix3x3::default(); spheres.len()];
    spheres.read(&mut vec).enq()?;

    println!("The value at index [{}] is now '{:?}'!", Float3::new(10.0,10.0,10.0), vec[50]);
    Ok(())
}



fn main() {
    
    trivial().unwrap();
    // println!("{}", ( Instant::now() -first).as_secs_f64());
    // trivial_explained().unwrap();
    // trivial_exploded().unwrap();
    // trivial_cored().unwrap();
}