#include <stddef.h>

#include "raylib.h"
#include "voxel.h"

int main(void)
{
  InitWindow(1280, 720, "voxel");
  SetTargetFPS(60);

  Camera3D camera = { 0 };
  camera.position = (Vector3){ 100.0f, 100.0f, 100.0f };
  camera.target = (Vector3){ 32.0f, 32.0f, 32.0f };
  camera.up = (Vector3){ 0.0f, 1.0f, 0.0f };     
  camera.fovy = 45.0f;         
  camera.projection = CAMERA_PERSPECTIVE;

  SetCameraMode(camera, CAMERA_FREE);

  Chunk *chunk = Chunk_init();

  for (size_t x = 0; x < CHUNK_SIZE; x++)
    for (size_t y = 0; y < CHUNK_SIZE; y++)
      for (size_t z = 0; z < CHUNK_SIZE; z++) {
        if ((x + y) % 2 == 0) {
          continue;
        }

        uint32_t hexColor = 0xFF | x << 8 | y << 16 | z << 24;

        Chunk_setBlock(chunk, hexColor, x, y, z);
      }

  while (!WindowShouldClose()) {
    UpdateCamera(&camera);

    BeginDrawing();
    ClearBackground(RAYWHITE);

    DrawFPS(10, 10);
    // DrawText("Hello World !", 190, 200, 20, LIGHTGRAY);

    BeginMode3D(camera);
    DrawGrid(100, 1.0f);

    for (size_t x = 0; x < CHUNK_SIZE; x++)
      for (size_t y = 0; y < CHUNK_SIZE; y++)
        for (size_t z = 0; z < CHUNK_SIZE; z++) {
          Block block;
          Chunk_getBlock(chunk , &block, x, y, z);

          if (block)
            DrawCube((Vector3){ .x = x, .y = y, .z = z }, 1.0f, 1.0f, 1.0f, GetColor(block));
        }
    

    EndMode3D();

    EndDrawing();
  }

  Chunk_cleanup(chunk);

  CloseWindow();

  return 0;
}