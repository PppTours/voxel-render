use crate::chunk::{Block, Chunk};

#[no_mangle]
pub extern "C" fn Chunk_init() -> Box<Chunk> {
    Box::new(Chunk::new())
}

#[no_mangle]
pub extern "C" fn Chunk_cleanup(chunk: Option<Box<Chunk>>) {
    drop(chunk);
}

#[no_mangle]
pub extern "C" fn Chunk_getBlockRef<'a>(
    chunk: Option<&'a mut Chunk>,
    referencece: Option<&mut &'a mut Block>,
    x: usize,
    y: usize,
    z: usize,
) -> bool {
    match (chunk, referencece) {
        (Some(chunk), Some(reference)) => {
            if let Some(block_ref) = chunk.get_ref_mut(x, y, z) {
                *reference = block_ref;
                true
            } else {
                false
            }
        }

        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn Chunk_getBlock(
    chunk: Option<&Chunk>,
    out: Option<&mut Block>,
    x: usize,
    y: usize,
    z: usize,
) -> bool {
    match (chunk, out) {
        (Some(chunk), Some(out)) => {
            if let Some(block) = chunk.get(x, y, z) {
                *out = block;

                true
            } else {
                false
            }
        }

        _ => false,
    }
}

#[no_mangle]
extern "C" fn Chunk_setBlock(
    chunk: Option<&mut Chunk>,
    input: Block,
    x: usize,
    y: usize,
    z: usize,
) -> bool {
    if let Some(chunk) = chunk {
        chunk.set(x, y, z, input);
        true
    } else {
        false
    }
}
