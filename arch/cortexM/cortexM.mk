OS   ?= bare
QEMU  = qemu-system-arm
APT  += gcc-arm-none-eabi gdb-multiarch openocd stlink-tools dfu-util $(QEMU)

.PHONY: qemu
qemu: $(ELF)
	$(QEMU) $(QEMU_CFG) -S -kernel $<
