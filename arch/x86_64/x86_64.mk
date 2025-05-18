OS       ?= linux
 TARGET   = $(ARCH)-unknown-$(OS)-gnu
RTARGET   = $(ARCH)-unknown-$(OS)-gnu
QEMU      = qemu-system-$(ARCH)
QEMU_CFG += -M type=q35,accel=kvm
