use ocl::prm::Float3;



#[repr(C,align(32))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Camera {
    pos : Float3,
    dir : Float3,
    fov : f32
}

unsafe impl ocl::OclPrm for Camera {}


impl Camera {
    pub fn new() -> Self {
        Self {
            pos : Float3::new(0.0,0.0,1.0),
            dir : Float3::new(0.0,0.0,-1.0),
            fov : 90.0
        }
    }
}