set(EXECUTABLE_NAME voxel)
set(SOURCES src/main.c)

add_executable(${EXECUTABLE_NAME} ${SOURCES})

add_subdirectory(raylib)
target_include_directories(${EXECUTABLE_NAME} PUBLIC raylib/src)
target_link_libraries(${EXECUTABLE_NAME} raylib voxel-render)