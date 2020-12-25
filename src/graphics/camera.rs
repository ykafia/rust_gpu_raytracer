use ocl::prm::Float4;


#[repr(C,packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Camera {
    pos : Float4,
    dir : Float4,
    fov : f32
}

unsafe impl ocl::OclPrm for Camera {}


impl Camera {
    pub fn new() -> Self {
        Self {
            pos : Float4::new(0.0,2.0,0.0,0.0),
            dir : Float4::new(0.0,0.0,-1.0,0.0),
            fov : 90.0
        }
    }
}