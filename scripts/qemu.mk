QEMU = qemu-system-$(ARCH)

MACHINE = -machine virt
BIOS = -bios $(UTILS_DIR)/rustsbi-qemu.bin
MOMERY = -m 128M
GRAPHIC = -nographic

QEMU_FLAGS = $(MACHINE) \
						 $(BIOS) \
						 $(MOMERY) \
						 $(GRAPHIC)

run: build
	$(QEMU) $(QEMU_FLAGS) -kernel $(BIN)

GDBINIT_TEMPLATE = $(WORK_DIR)/.gdbinit.template
GDBINIT_FILE = $(WORK_DIR)/.gdbinit
GDB_PORT = 27000
GDB_QEMU_FLAGS = -S -gdb tcp::$(GDB_PORT)

gdb: build
	@grep -E "set auto-load safe-path /" ~/.gdbinit || echo "set auto-load safe-path /" >> ~/.gdbinit
	@cp $(GDBINIT_TEMPLATE) $(GDBINIT_FILE)
	@echo "target remote localhost:$(GDB_PORT)\nfile $(BIN)" >> $(GDBINIT_FILE)
	$(QEMU) $(QEMU_FLAGS) -kernel $(BIN) $(GDB_QEMU_FLAGS)

.PHONY: run gdb
