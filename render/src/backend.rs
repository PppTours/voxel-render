use std::ffi::{c_void, CString};

use glium::backend::Backend;

pub struct VoxelRenderBackend {
    pub procfn: extern "C" fn(symbol_ptr: *const i8) -> *const c_void,
    pub framebuffer_size: (u32, u32),
}

unsafe impl Backend for VoxelRenderBackend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        let cstr = CString::new(symbol).unwrap();

        (self.procfn)(cstr.as_ptr())
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        // required ?
        self.framebuffer_size
    }

    fn is_current(&self) -> bool {
        // Assume we are always in the current context.
        true
    }

    unsafe fn make_current(&self) {}
}
