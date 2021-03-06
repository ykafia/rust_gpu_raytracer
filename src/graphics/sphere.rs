
use nalgebra::Vector3;
use ocl::{OclPrm, prm::Float4};
use rand::Rng;

use super::Material;

// use crate::maths::*;



#[repr(C,align(64))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere{
    pub pos : Float4,
    pub radius : f32,
    pub material : Material
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new_random()
    }
}

impl Sphere {
    pub fn new(pos : Float4, radius : f32, color : Float4, reflect : f32) -> Self{
        Self {
            pos,
            radius,
            material : Material {
                color,
                reflectivity : reflect,
                albedo : 1.0
            }
        }
    }
    pub fn new_random() -> Self{
        let mut rng = rand::thread_rng();
        Self::new(
            Float4::new(rng.gen::<f32>() % 5.0,rng.gen::<f32>() % 5.0,rng.gen::<f32>() % 2.0,0.0),
            rng.gen::<f32>() % 2.0,
            Float4::new(rng.gen::<f32>() %1.0,rng.gen::<f32>()%1.0,rng.gen::<f32>()%1.0,0.0),
            rng.gen::<f32>() %1.0
        )
    }
    pub fn new_cyan(pos : Vector3<f32>) -> Self{
        Self {
            pos : Float4::new(pos.x,pos.y,pos.z,0.0),
            radius : 1.0,
            material : Material {
                color : Float4::new(0.0,0.8,1.0,0.0),
                reflectivity : 0.0,
                albedo : 1.0
            }
        }
    }
    pub fn new_red(pos : Vector3<f32>) -> Self{
        Self {
            pos : Float4::new(pos.x,pos.y,pos.z,0.0),
            radius : 1.0,
            material : Material {
                color : Float4::new(1.0,0.0,0.0,0.0),
                reflectivity : 0.0,
                albedo : 1.0
            }
        }
    }
}

unsafe impl OclPrm for Sphere{

}