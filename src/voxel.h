#ifndef H_VOXEL
#define H_VOXEL

#include <stdbool.h>
#include <stdint.h>

#define CHUNK_SIZE 64
#define REGION_SIZE 8

typedef uint64_t Block;

// Represents a chunk, which is a renderable unit.
typedef struct Chunk {
  // If the chunk has been allocated.
  bool isAllocated;

  // True if the chunk has no block, serves as an optimization.
  bool isEmpty;

  // Large array of blocks allocated.
  // allocation size = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * sizeof(Block)
  Block *data;
  
  // ...
} Chunk;

// Represents a region which is a container for chunks.
typedef struct Region {
  struct Chunk chunks[REGION_SIZE * REGION_SIZE * REGION_SIZE];
} Region;

// Initialize a chunk.
bool Chunk_init(Chunk *self);

// Cleanup a chunk.
void Chunk_cleanup(Chunk *self);

// Get the pointer to a block in a chunk.
bool Chunk_getBlockRef(const Chunk *self, Block **reference, size_t x, size_t y, size_t z);

// Get the block value in a chunk.
bool Chunk_getBlock(const Chunk *self, Block *out, size_t x, size_t y, size_t z);

// Set a block value in a chunk.
bool Chunk_setBlock(const Chunk *self, const Block *in, size_t x, size_t y, size_t z);

#endif
