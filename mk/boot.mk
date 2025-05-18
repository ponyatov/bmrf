APT += syslinux isolinux xorriso

ISOLINUX += root/isolinux/isohdpfx.bin
ISOLINUX += root/isolinux/isohdppx.bin
ISOLINUX += root/isolinux/isolinux.bin

ISOLINUX += root/EFI/BOOT/ldlinux.e64
ISOLINUX += root/EFI/BOOT/syslinux.c32

.PHONY: isolinux
isolinux: bin/$(BINFILE).iso

.PHONY: iso
iso: bin/$(BINFILE).iso
bin/$(BINFILE).iso: $(ISOLINUX) mk/boot.mk
	xorriso -as mkisofs -o $@ -r root -V $(BINFILE) \
	-isohybrid-mbr root/isolinux/isohdpfx.bin -b isolinux/isolinux.bin \
	-c isolinux/boot.cat -boot-load-size 4 -boot-info-table -no-emul-boot

root/isolinux/%: /usr/lib/ISOLINUX/%
	cp $< $@
root/EFI/BOOT/%: /usr/lib/syslinux/modules/efi64/%
	cp $< $@

.PHONY: qemu
qemu: bin/$(BINFILE).iso
	$(QEMU) -boot d -cdrom $<
