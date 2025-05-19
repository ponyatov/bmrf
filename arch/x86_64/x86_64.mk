include arch/x86/x86.mk

OS       ?= linux
 TARGET   = $(ARCH)-unknown-$(OS)-gnu
RTARGET   = $(ARCH)-unknown-$(OS)-gnu
QEMU      = qemu-system-$(ARCH)
QEMU_CFG += -M type=q35,accel=kvm

.PHONY: qemu
qemu: bin/$(BINFILE).iso
	$(QEMU) -boot d -cdrom $<
