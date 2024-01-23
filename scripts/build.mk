CARGO = cargo

MODE ?= debug

BIN = $(TARGET_DIR)/$(TARGET)/$(MODE)/$(PROJECT)

build:
	$(CARGO) build

clean:
	$(CARGO) clean

.PHONY: build clean
