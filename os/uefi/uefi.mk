APT      += ovmf ovmf-ia32
QEMU_CFG += -bios /usr/share/ovmf/OVMF.fd

 TARGET = $(ARCH)-unknown-$(OS)-gnu
RTARGET = $(ARCH)-unknown-uefi
ifeq ($(ARCH),i386)
 TARGET = i686-unknown-$(OS)-gnu
RTARGET = i686-unknown-uefi
endif
