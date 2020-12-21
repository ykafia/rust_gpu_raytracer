use std::fmt;

use ocl::OclPrm;

#[repr(C)]
#[derive(Debug,Copy,Default,Clone, PartialEq)]
pub struct Float3 {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Float3 {
    pub fn new(x : f32, y : f32, z : f32,) -> Self{
        Self {
            x,
            y,
            z
        }
    }
}

unsafe impl OclPrm for Float3{
    
}

impl fmt::Display for Float3 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "[{};{};{}]", self.x, self.y, self.z)
    }
}