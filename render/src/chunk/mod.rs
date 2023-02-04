mod buffer;

pub const CHUNK_SIZE: usize = 16;
pub type Block = u64;

pub type ChunkData = [u64; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];

pub struct Chunk {
    pub data: Option<Box<ChunkData>>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            data: Some(Box::new(
                [Block::default(); CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            )),
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_ref(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
        if let Some(data) = &self.data {
            data.get(x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE)
        } else {
            None
        }
    }

    pub fn get_ref_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut Block> {
        if let Some(data) = &mut self.data {
            data.get_mut(x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE)
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<Block> {
        self.get_ref(x, y, z).copied()
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        if let Some(r) = self.get_ref_mut(x, y, z) {
            *r = block
        }
    }
}
