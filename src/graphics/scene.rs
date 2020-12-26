use std::error::{self, Error};

use nalgebra::Vector3;
use ocl::{Buffer, OclPrm, ProQue, Result, prm::{Float2, Float3}};

use crate::util::read_file;

use super::{RenderInfo, Camera, DirectionalLight, Plane, Sphere, directional_light};

pub struct Scene {
    pub width : u32,
    pub height : u32,
    pub kernel : ocl::Kernel,
    pub camera : Camera,
    pub spheres : Vec<Sphere>,
    pub spheres_buf : Buffer<Sphere>,
    pub planes : Vec<Plane>,
    pub planes_buf : Buffer<Plane>,
    pub directional_lights : Vec<DirectionalLight>,
    pub directional_lights_buf : Buffer<DirectionalLight>,
    pub bufi : RenderInfo,
    pub screen : Vec<u32>,
    pub screen_buf : Buffer<u32>
}

impl Scene {

    pub fn new(width : u32, height : u32) -> Self {
        // let src = r#"
        //     __kernel 
        //     void add(__global float2* buffer, float scalar) {
        //         buffer[get_global_id(0)] += scalar;
        //     }
        // "#;
    
        let src = read_file("./src/kernels/ray_trace.ocl");
    
        let pro_que = ProQue::builder()
            .src(src)
            .dims(1 << 21)
            .build().unwrap();
        

        let sphere_pos = Vector3::new(0.0,1.0,0.0);
        let cam_pos = Vector3::new(0.0,1.0,-5.0);

        let camera = Camera::new(cam_pos,sphere_pos-cam_pos);

        let screen = vec![0u32;(height*width) as usize];
        let spheres = vec![Sphere::new_cyan(sphere_pos)];
        let planes = vec![Plane::default()];
        let directional_lights = vec![DirectionalLight::default()];

        let screen_buf = pro_que.create_buffer::<u32>().unwrap();
        let spheres_buf = pro_que.create_buffer::<Sphere>().expect("Sphere buffer badly create");
        let directional_lights_buf = pro_que.create_buffer::<DirectionalLight>().unwrap();
        let planes_buf = pro_que.create_buffer::<Plane>().unwrap(); 
        

        spheres_buf.write(&spheres).enq().unwrap();
        directional_lights_buf.write(&directional_lights).enq().unwrap();
        screen_buf.write(&screen).enq().unwrap();

        // let bufi_buffer = pro_que.create_buffer::<BufferInfo>().unwrap(); 
        let bufi = RenderInfo::new(
            spheres.len() as u32,
            planes.len() as u32,
            directional_lights.len() as u32,
            height,
            width
        );
        
        

        let kernel = pro_que.kernel_builder("ray_trace")
            .arg(&screen_buf)
            .arg(camera)
            .arg(bufi)
            .arg(&spheres_buf)
            .arg(&planes_buf)
            .arg(&directional_lights_buf)
            .build().unwrap();
    
        
        Self {
            width,
            height,
            kernel,
            camera,
            spheres,
            spheres_buf,
            planes,
            planes_buf,
            directional_lights,
            directional_lights_buf,
            bufi,
            screen,
            screen_buf,
        }
    

    }
    pub fn compute(&mut self) -> ocl::Result<()>{
        unsafe { self.kernel.enq()?; }
    
        Ok(())
    }
    pub fn get_screen(&mut self) -> &Vec<u32> {
        self.screen_buf.read(&mut self.screen).enq().unwrap();
        &self.screen
    }
    // pub fn update_spheres(&mut self) -> Result<()> {
    //     self.spheres_buf.write(&self.spheres).enq()?;
    //     Ok(())
    // }
}