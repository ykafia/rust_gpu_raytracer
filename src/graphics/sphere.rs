use ocl::{OclPrm, prm::{Float, Float2, Float3}};

// use crate::maths::Float3;



#[repr(C)]
#[derive(Debug,Copy,Default,Clone, PartialEq)]
pub struct Sphere(pub Float3,pub Float3);

unsafe impl OclPrm for Sphere{

}