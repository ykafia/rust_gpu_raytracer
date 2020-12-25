use ocl::{OclPrm, prm::Float4};

use super::Material;


#[repr(C,align(32))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub pos : Float4,
    pub normal : Float4,
    pub material : Material
}

impl Default for Plane {
    fn default() -> Self {
        Self{
            pos : Float4::new(0.0,-2.0,0.0,0.0),
            normal : Float4::new(0.0,-1.0,0.0,0.0),
            material : Material {
                color: Float4::new(0.2,0.2,0.2,0.0),
                reflectivity: 0.5,
                albedo: 1.0,

            }
        }
    }
}

unsafe impl OclPrm for Plane {}