include arch/x86/x86.mk

APT      += gcc-i686-linux-gnu $(QEMU)

OS       ?= uefi
 TARGET   = i686-unknown-$(OS)-gnu
RTARGET   = i686-unknown-$(OS)-gnu
QEMU_CFG += -M type=pc,accel=kvm

.PHONY: qemu
qemu: iso
