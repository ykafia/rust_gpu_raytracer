use ocl::OclPrm;

#[repr(C,align(64))]
#[derive(Debug,Default,Clone, Copy,PartialEq)]
pub struct Matrix3x3(pub [f32;9]);

unsafe impl OclPrm for Matrix3x3{}