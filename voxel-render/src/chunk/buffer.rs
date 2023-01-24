use glium::buffer::{Buffer, BufferCreationError, BufferMode, BufferType};

use crate::{chunk::ChunkData, VoxelRender};

pub(super) struct ChunkBuffer(pub Buffer<ChunkData>);

impl ChunkBuffer {
    pub fn new(r: &VoxelRender, data: ChunkData) -> Result<Self, BufferCreationError> {
        Buffer::new(r, &data, BufferType::UniformBuffer, BufferMode::Default)
            .map(|buffer| ChunkBuffer(buffer))
    }
}
