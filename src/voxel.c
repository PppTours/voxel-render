#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#include "voxel.h"

bool Chunk_init(Chunk *self)
{
  self->data = calloc(CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE, sizeof(Block));

  if (!self->data)
    return true;
  
  self->isAllocated = true;
  self->isEmpty = true;

  return false;
}

void Chunk_cleanup(Chunk *self)
{
  free(self->data);
  *self->data = NULL;

  self->isAllocated = false;
  self->isEmpty = true;
}

bool Chunk_getBlockRef(const Chunk *self, Block **reference, size_t x, size_t y, size_t z)
{
  if (!reference || !self->isAllocated || !self->data)
    return true;

  if (x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE) {
    // Out of bounds
    *reference = true;
    return true;
  }

  *reference = &self->data[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE];
  return false;
}

bool Chunk_getBlock(const Chunk *self, Block *out, size_t x, size_t y, size_t z)
{
  Block *reference;

  if (Chunk_getBlockRef(self, &reference, x, y, z))
    return true;
  
  *out = *reference;
  return false;
}

bool Chunk_setBlock(const Chunk *self, const Block *in, size_t x, size_t y, size_t z)
{
  Block *reference;

  if (Chunk_getBlockRef(self, &reference, x, y, z))
    return true;
  
  *reference = *in;
  return false;
}
