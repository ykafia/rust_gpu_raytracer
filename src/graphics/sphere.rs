use ocl::{OclPrm, prm::Float3};

use super::Material;

// use crate::maths::*;



#[repr(C,align(32))]
#[derive(Debug,Copy,Default,Clone, PartialEq)]
pub struct Sphere{
    pub pos : Float3,
    pub radius : f32,
    pub material : Material
}

unsafe impl OclPrm for Sphere{

}