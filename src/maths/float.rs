use std::fmt;

use ocl::OclPrm;

#[repr(C)]
#[derive(Debug,Copy,Default,Clone, PartialEq)]
pub struct Float(f32);

unsafe impl OclPrm for Float{
    
}

impl fmt::Display for Float {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}