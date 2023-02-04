CFLAGS := -O2 -s
LDFLAGS := -O2 -s -lm

SRC := src/voxel.c
OBJ := $(SRC:.c=.o)

RENDER_PROFILE ?= debug

CFLAGS += -Iraylib/src
LDFLAGS += raylib/src/libraylib.a

# raylib settings
PLATFORM ?= PLATFORM_DESKTOP
GRAPHICS ?= GRAPHICS_API_OPENGL_42

CFLAGS += -D$(GRAPHICS) -D$(PLATFORM)

USE_WAYLAND_DISPLAY ?= FALSE
USE_EXTERNAL_GLFW ?= FALSE

ifeq ($(OS),Windows_NT)
	LDFLAGS += -lopengl32 -lgdi32 -lwinmm -static
else ifeq ($(shell uname),Darwin)
	LDFLAGS += -framework CoreVideo -framework IOKit -framework Cocoa -framework GLUT -framework OpenGL
else
	LDFLAGS += -ldl -lpthread -lX11
	EXTERNAL_FILES :=
endif

all: raylib voxel

%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS)

raylib:
	$(MAKE) -C raylib/src \
		CC=$(CC) AR=$(AR) CFLAGS="$(CFLAGS)" LDFLAGS="$(LDFLAGS)" \
		USE_WAYLAND_DISPLAY="$(USE_WAYLAND_DISPLAY)" \
		USE_EXTERNAL_GLFW="$(USE_EXTERNAL_GLFW)" \
		PLATFORM="$(PLATFORM)" GRAPHICS="$(GRAPHICS)"

voxel: src/main.c ../render/target/$(RENDER_PROFILE)/libvoxel_render.a
	$(CC) -o $@ $^ $(LDFLAGS)

clean:
	rm -rf voxel $(OBJ)
	$(MAKE) -C raylib/src clean

.PHONY: all voxel raylib clean
