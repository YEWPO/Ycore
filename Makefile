PROJECT = yros

WORK_DIR = $(abspath .)

SCRIPT_DIR = $(WORK_DIR)/scripts
UTILS_DIR = $(WORK_DIR)/utils
TARGET_DIR = $(WORK_DIR)/target

ARCH = riscv64
TARGET = riscv64imac-unknown-none-elf

include $(SCRIPT_DIR)/build.mk
include $(SCRIPT_DIR)/qemu.mk
