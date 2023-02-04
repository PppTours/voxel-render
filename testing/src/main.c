#include <stddef.h>

#include "raylib.h"
#include "raymath.h"
#include "rlgl.h"
#include "voxel.h"

#include "GLFW/glfw3.h"

int main(void)
{
  InitWindow(1280, 720, "voxel");
  SetTargetFPS(60);

  Camera3D camera = { 0 };
  camera.position = (Vector3){ 25.0f, 25.0f, 25.0f };
  camera.target = (Vector3){ 0.0f, 0.0f, 0.0f };
  camera.up = (Vector3){ 0.0f, 1.0f, 0.0f };     
  camera.fovy = 45.0f;         
  camera.projection = CAMERA_PERSPECTIVE;

  SetCameraMode(camera, CAMERA_FREE);

  VoxelRender *render = VoxelRender_init(glfwGetProcAddress, 1280, 720);
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
    DrawText("Hello World !", 190, 200, 20, LIGHTGRAY);

    BeginMode3D(camera);
    DrawGrid(100, 1.0f);

    rlDisableBackfaceCulling();
    rlDrawRenderBatchActive();
    Matrix matrix = MatrixMultiply(rlGetMatrixModelview(), rlGetMatrixProjection());
    VoxelRender_drawCube(render, &matrix);
    
    EndMode3D();

    EndDrawing();
  }

  Chunk_cleanup(chunk);
  VoxelRender_cleanup(render);

  CloseWindow();

  return 0;
}