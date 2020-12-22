use ocl::{OclPrm, prm::Float3};


#[repr(C,align(32))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub pos : Float3,
    pub normal : Float3
}

impl Default for Plane {
    fn default() -> Self {
        Self{
            pos : Float3::new(0.0,0.0,0.0),
            normal : Float3::new(0.0,1.0,0.0)
        }
    }
}

unsafe impl OclPrm for Plane {}