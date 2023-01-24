use std::ffi::c_void;

use crate::{backend::VoxelRenderBackend, VoxelRender};

mod chunk;

#[no_mangle]
pub extern "C" fn VoxelRender_init(
    procfn: extern "C" fn(*const i8) -> *const c_void,
    width: u32,
    height: u32,
) -> Box<VoxelRender> {
    Box::new(VoxelRender::new(VoxelRenderBackend {
        procfn,
        framebuffer_size: (width, height),
    }))
}

#[no_mangle]
pub extern "C" fn VoxelRender_cleanup(ctx: Option<Box<VoxelRender>>) {
    drop(ctx);
}
