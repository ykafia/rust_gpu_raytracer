use std::error::{self, Error};

use ocl::{Buffer, OclPrm, ProQue, prm::{Float2, Float3}};

use crate::util::read_file;

use super::{Camera, Sphere};

pub struct Scene {
    pub width : u32,
    pub height : u32,
    pub kernel : ocl::Kernel,
    pub camera : Vec<Camera>,
    pub camera_buf : Buffer<Camera>,
    pub spheres : Vec<Sphere>,
    pub spheres_buf : Buffer<Sphere>,
    pub screen : Vec<u32>,
    pub screen_buf : Buffer<u32>
}

impl Scene {

    pub fn new(width : u32, height : u32) -> ocl::Result<Self> {
        // let src = r#"
        //     __kernel 
        //     void add(__global float2* buffer, float scalar) {
        //         buffer[get_global_id(0)] += scalar;
        //     }
        // "#;
    
        let src = read_file("./src/kernels/Compute.ocl");
    
        let pro_que = ProQue::builder()
            .src(src)
            .dims(1 << 20)
            .build()?;
        
        let spheres = vec![Sphere::new_cyan()];
        let spheres_buf = pro_que.create_buffer::<Sphere>()?;
        let screen_buf = pro_que.create_buffer::<u32>()?;
        let camera = vec![Camera::new()];
        let camera_buf = pro_que.create_buffer::<Camera>()?;

        spheres_buf.write(&spheres).enq()?;
        camera_buf.write(&camera).enq()?;
        
        let kernel = pro_que.kernel_builder("compute")
            .arg(&screen_buf)    
            .arg(&spheres_buf)
            .arg(&camera_buf)
            .arg(height)
            .arg(width)
            .build()?;
    
        Ok(
            Self {
                width,
                height,
                kernel,
                camera,
                camera_buf,
                spheres,
                spheres_buf,
                screen : vec![0;(width*height) as usize],
                screen_buf,
            }
        )

    }
    pub fn compute(&mut self) -> ocl::Result<()>{
        unsafe { self.kernel.enq()?; }
    
        Ok(())
    }
    pub fn get_screen(&mut self) -> &Vec<u32> {
        self.screen_buf.read(&mut self.screen).enq().unwrap();
        &self.screen
    }
}