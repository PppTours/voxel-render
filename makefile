PROFILE ?= debug

ifeq ($(PROFILE),debug)
	RUST_PROFILE=dev
else
	RUST_PROFILE=release
endif

all: test

render:
	cd render && cargo build --profile $(RUST_PROFILE)

testing:
	$(MAKE) -C testing RENDER_PROFILE=$(PROFILE)

test: render testing
	testing/voxel

debug: render testing
	gdb testing/voxel

clean:
	cd render && cargo clean
	$(MAKE) -C testing clean

.PHONY: render testing test