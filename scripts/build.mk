CARGO = cargo

MODE ?= debug
export LOG ?= info

BIN = $(TARGET_DIR)/$(TARGET)/$(MODE)/$(PROJECT)

build:
	$(CARGO) build

clean:
	$(CARGO) clean

.PHONY: build clean
