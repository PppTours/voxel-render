#ifndef H_VOXEL
#define H_VOXEL

#include <raylib.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define CHUNK_SIZE 64
#define REGION_SIZE 8

typedef uint64_t Block;

// Represents a chunk, which is a renderable unit.
typedef struct Chunk /*{
  // If the chunk has been allocated.
  bool isAllocated;

  // True if the chunk has no block, serves as an optimization.
  bool isEmpty;

  // Large array of blocks allocated.
  // allocation size = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * sizeof(Block)
  Block *data;
  
  // ...
}*/ Chunk;

// Represents a region which is a container for chunks.
#if 0
typedef struct Region {
  struct Chunk chunks[REGION_SIZE * REGION_SIZE * REGION_SIZE];
} Region;
#endif

// Initialize a chunk.
Chunk *Chunk_init(void);

// Cleanup a chunk.
void Chunk_cleanup(Chunk *self);

// Get the pointer to a block in a chunk.
bool Chunk_getBlockRef(const Chunk *self, Block **reference, size_t x, size_t y, size_t z);

// Get the block value in a chunk.
bool Chunk_getBlock(const Chunk *self, Block *out, size_t x, size_t y, size_t z);

// Set a block value in a chunk.
bool Chunk_setBlock(Chunk *self, const Block in, size_t x, size_t y, size_t z);

typedef struct VoxelRender VoxelRender; // opaque

VoxelRender *VoxelRender_init(void *procfn, uint32_t width, uint32_t height); // width/height succeptibles d'être enlevés^M

void VoxelRender_cleanup(VoxelRender *render);

void VoxelRender_drawCube(VoxelRender *render, Matrix *mat);

#endif
