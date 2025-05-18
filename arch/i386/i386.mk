OS       ?= uefi
QEMU      = qemu-system-$(ARCH)
 TARGET   = i686-unknown-$(OS)-gnu
RTARGET   = i686-unknown-$(OS)-gnu
APT      += gcc-i686-linux-gnu
QEMU_CFG += -M type=pc,accel=kvm
