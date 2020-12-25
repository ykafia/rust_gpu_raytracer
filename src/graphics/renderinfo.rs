use ocl::OclPrm;



#[repr(C,packed)]
#[derive(Debug,Default,Clone, Copy,PartialEq)]
pub struct RenderInfo {
    pub sphere_number : u32,
    pub plane_number : u32,
    pub light_number : u32,
    pub height : u32,
    pub width : u32
}

impl RenderInfo {
    pub fn new(sphere_number : u32,plane_number : u32,light_number : u32, height : u32, width : u32) -> Self {
        Self {
            sphere_number,
            plane_number,
            light_number,
            height,
            width
        }
    }
}

unsafe impl OclPrm for RenderInfo{}