add_executable(voxel src/main.c)

add_subdirectory(raylib)
target_include_directories(voxel PUBLIC raylib/src)
target_link_libraries(voxel raylib voxel-render)