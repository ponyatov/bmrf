HW ?= pc
# HW ?= lm3s6
# HW ?= pillF103

ELF = bin/$(BINFILE).elf
DFU = bin/$(BINFILE).dfu

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

.PHONY: i486
i486:
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
	cargo +nightly build -Zbuild-std --target .cargo/i486-pc-bare.json
