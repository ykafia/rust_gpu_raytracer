use ocl::{OclPrm, prm::Float4};

#[repr(C,align(8))]
#[derive(Debug,Default,Clone, Copy,PartialEq)]
pub struct Material{
    pub color : Float4,
    pub reflectivity : f32
}

unsafe impl OclPrm for Material{}