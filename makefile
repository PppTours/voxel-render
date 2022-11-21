CFLAGS := -O2 -s
LDFLAGS := -O2 -s -lm

AR ?= ar

SRC := src/voxel.c

OBJ := $(SRC:.c=.o)

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
	LDFLAGS_R += -mwindows
else ifeq ($(shell uname),Darwin)
	LDFLAGS += -framework CoreVideo -framework IOKit -framework Cocoa \
		-framework GLUT -framework OpenGL \
		-Wl,-pagezero_size,10000,-image_base,100000000
else
	LDFLAGS += -ldl -lpthread
	ifeq ($(PLATFORM),PLATFORM_DRM)
		LDFLAGS += -ldrm -lGLESv2 -lEGL -lgbm
	else
		LDFLAGS += -lX11
	endif
	EXTERNAL_FILES :=
endif

all: raylib libvoxel.a voxel

%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS)

raylib:
	$(MAKE) -C raylib/src \
		CC=$(CC) AR=$(AR) CFLAGS="$(CFLAGS)" LDFLAGS="$(LDFLAGS)" \
		USE_WAYLAND_DISPLAY="$(USE_WAYLAND_DISPLAY)" \
		USE_EXTERNAL_GLFW="$(USE_EXTERNAL_GLFW)" \
		PLATFORM="$(PLATFORM)" GRAPHICS="$(GRAPHICS)"

voxel: src/main.c libvoxel.a
	$(CC) -o $@ $^ $(LDFLAGS)

libvoxel.a: $(OBJ)
	$(AR) rcu $@ $^

clean:
	rm -rf voxel libvoxel.a $(OBJ)
	$(MAKE) -C raylib/src clean
	rm -f raylib/libraylib.a

.PHONY: all voxel libvoxel.a raylib clean
