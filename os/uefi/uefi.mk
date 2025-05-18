APT      += ovmf ovmf-ia32 isolinux
QEMU_CFG += -m 128M -bios /usr/share/ovmf/OVMF.fd
QEMU_CFG += -nic none

 TARGET = $(ARCH)-unknown-$(OS)-gnu
RTARGET = $(ARCH)-unknown-uefi
ifeq ($(ARCH),i386)
 TARGET = i686-unknown-$(OS)-gnu
RTARGET = i686-unknown-uefi
endif

.PHONY: uefi
uefi: bin/$(BINFILE).iso
	$(QEMU) $(QEMU_CFG) -boot d -cdrom $<
