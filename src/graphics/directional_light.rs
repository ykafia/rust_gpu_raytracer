use ocl::{OclPrm, prm::{Float4}};


#[repr(C,align(64))]
#[derive(Debug,Clone, Copy, PartialEq)]
pub struct DirectionalLight {
    pub direction : Float4,
    pub color : Float4,
    pub intensity : f32
}

impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            direction : Float4::new(0.0,-1.0, 1.0,0.0),
            color : Float4::new(1.0,1.0,1.0,0.0),
            intensity : 1.0
        }
    }
}

unsafe impl OclPrm for DirectionalLight {}