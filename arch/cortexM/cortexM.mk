OS   ?= bare
QEMU  = qemu-system-arm
APT  += gdb-multiarch gcc-arm-none-eabi stlink-tools dfu-util $(QEMU)

.PHONY: qemu
qemu: $(ELF)
	$(QEMU) $(QEMU_CFG) -gdb tcp::12345 -S -kernel $<
