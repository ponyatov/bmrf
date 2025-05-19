HW ?= pc
# HW ?= lm3s6
# HW ?= pillF103

ELF = bin/$(BINFILE).elf
DFU = bin/$(BINFILE).dfu

QEMU_CFG += -gdb tcp::12345

include   hw/$(HW)/$(HW).mk
include  cpu/$(CPU)/$(CPU).mk
include arch/$(ARCH)/$(ARCH).mk
include   os/$(OS)/$(OS).mk

.PHONY: elf
elf: $(ELF)

.PHONY: dfu
dfu: $(DFU)
$(DFU): $(ELF)
	~/elf2dfuse/bin/elf2dfuse $< $@
