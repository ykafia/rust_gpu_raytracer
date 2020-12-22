use ocl::{OclPrm, prm::{Float3, Float4}};
use rand::Rng;

use super::Material;

// use crate::maths::*;



#[repr(C,align(32))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere{
    pub pos : Float3,
    pub radius : f32,
    pub material : Material
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new_random()
    }
}

impl Sphere {
    pub fn new(pos : Float3, radius : f32, color : Float4, reflect : f32) -> Self{
        Self {
            pos,
            radius,
            material : Material {
                color,
                reflectivity : reflect
            }
        }
    }
    pub fn new_random() -> Self{
        let mut rng = rand::thread_rng();
        Self::new(
            Float3::new(rng.gen::<f32>() % 5.0,rng.gen::<f32>() % 5.0,rng.gen::<f32>() % 2.0),
            rng.gen::<f32>() % 2.0,
            Float4::new(rng.gen::<f32>() %1.0,rng.gen::<f32>()%1.0,rng.gen::<f32>()%1.0,1.0),
            rng.gen::<f32>() %1.0
        )
    }
    pub fn new_cyan() -> Self{
        Self {
            pos : Float3::new(0.0,0.0,5.0),
            radius : 1.0,
            material : Material {
                color : Float4::new(0.0,1.0,1.0,1.0),
                reflectivity : 0.0
            }
        }
    }
}

unsafe impl OclPrm for Sphere{

}