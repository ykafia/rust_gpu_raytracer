use ocl::Buffer;

use super::Sphere;

pub struct Scene {
    pub width : u32,
    pub height : u32,
    pub spheres : Buffer<Sphere>,
    pub screen : Buffer<u32>
}

impl Scene {

    pub fn new() {
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
}