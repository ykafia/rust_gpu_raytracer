use std::fmt;

use ocl::OclPrm;

#[repr(C,packed)]
#[derive(Debug,Copy,Default,Clone, PartialEq)]
pub struct Float4 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32
}

impl Float4{
    pub fn new(x : f32, y : f32, z : f32, w : f32) -> Self{
        Self {
            x,
            y,
            z,
            w
        }
    }
}

unsafe impl OclPrm for Float4{
    
}

impl fmt::Display for Float4 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "[{};{};{};{}]", self.x, self.y, self.z, self.w)
    }
}