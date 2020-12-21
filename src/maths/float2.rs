use std::fmt;

use ocl::OclPrm;

#[repr(C)]
#[derive(Debug,Copy,Default,Clone, PartialEq)]
pub struct Float2 {
    pub x : f32,
    pub y : f32
}

impl Float2 {
    pub fn new(x : f32, y : f32) -> Self{
        Self {
            x,
            y
        }
    }
}

unsafe impl OclPrm for Float2{
    
}

impl fmt::Display for Float2 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "[{};{}]", self.x, self.y)
    }
}