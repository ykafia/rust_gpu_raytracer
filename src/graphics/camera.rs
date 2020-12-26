use nalgebra::Vector3;
use ocl::{OclPrm, prm::Float4};


#[repr(C,packed)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Camera {
    pub pos : Float4,
    pub dir : Float4,
    pub fov : f32
}

unsafe impl ocl::OclPrm for Camera {}


impl Camera {
    pub fn new(pos : Vector3<f32>, dir : Vector3<f32>) -> Self {
        Self {
            pos : Float4::new(0.0,1.0,-5.0,0.0),
            dir : Float4::new(dir.x,dir.y,dir.z,0.0),
            fov : 80.0
        }
    }
    pub fn face_towards(&mut self, target : Float4) {
        self.dir = target - self.pos;
    }
}