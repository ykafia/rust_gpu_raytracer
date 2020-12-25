use ocl::{OclPrm, prm::Float4};



#[repr(C,packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material{
    pub color : Float4,
    pub reflectivity : f32,
    pub albedo : f32
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color : Float4::new(0.0,1.0,1.0,0.0),
            reflectivity : 0.0,
            albedo : 1.0,
        }
    }
}

unsafe impl OclPrm for Material{}