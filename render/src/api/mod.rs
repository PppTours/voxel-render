use std::{array::from_fn, ffi::c_void, ptr::NonNull};

use crate::{backend::VoxelRenderBackend, VoxelRender};

mod chunk;

#[no_mangle]
pub extern "C" fn VoxelRender_init(
    procfn: extern "C" fn(*const i8) -> *const c_void,
    width: u32,
    height: u32,
) -> *mut VoxelRender {
    Box::leak(Box::new(VoxelRender::new(VoxelRenderBackend {
        procfn,
        framebuffer_size: (width, height),
    }))) as *mut _
}

#[no_mangle]
pub extern "C" fn VoxelRender_cleanup(ctx: Option<NonNull<*mut VoxelRender>>) {
    if let Some(ctx_ptr) = ctx {
        drop(unsafe { Box::from_raw(ctx_ptr.as_ptr()) });
    }
}

#[no_mangle]
pub extern "C" fn VoxelRender_drawCube(ctx: Option<&mut VoxelRender>, mvp: *const f32) {
    if let Some(ctx) = ctx {
        // HACK: do something better ?
        let mat = from_fn(|i| from_fn(|j| unsafe { mvp.add(i + j * 4).read() }));

        ctx.draw_cube(mat);
    }
}
