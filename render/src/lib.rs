use std::rc::Rc;

use cube::CubeVertex;
use glium::{
    backend::{Context, Facade},
    debug::DebugCallbackBehavior,
    framebuffer::DefaultFramebuffer,
    uniform, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};

mod api;
pub mod backend;
pub mod chunk;
mod cube;

pub struct VoxelRender {
    pub(crate) context: Rc<Context>,
    pub(crate) cube_obj: (VertexBuffer<CubeVertex>, IndexBuffer<u8>),
    pub(crate) basic_program: Program,
}

impl Facade for VoxelRender {
    fn get_context(&self) -> &Rc<Context> {
        &self.context
    }
}

const VERTEX_SHADER: &'static str = include_str!("shaders/basic.vert");
const FRAG_SHADER: &'static str = include_str!("shaders/basic.frag");

impl VoxelRender {
    pub fn new(backend: backend::VoxelRenderBackend) -> VoxelRender {
        let context =
            unsafe { Context::new(backend, false, DebugCallbackBehavior::PrintAll) }.unwrap();

        println!(
            "Initializing voxel renderer... (OpenGL version: {})",
            context.get_opengl_version_string()
        );

        VoxelRender {
            cube_obj: cube::gen_cube(&context),
            basic_program: Program::from_source(&context, VERTEX_SHADER, FRAG_SHADER, None)
                .unwrap(),
            context,
        }
    }

    pub fn draw_cube(&self, mvp: [[f32; 4]; 4]) {
        unsafe {
            self.get_context().reset_state();
        }

        let mvp_uniform = uniform! {
            mvp: mvp
        };

        let mut fb = DefaultFramebuffer::back_left(self);

        fb.draw(
            &self.cube_obj.0,
            &self.cube_obj.1,
            &self.basic_program,
            &mvp_uniform,
            &DrawParameters::default()
        )
        .unwrap();
    }
}
