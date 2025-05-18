APT += syslinux isolinux xorriso

ISOLINUX += root/isolinux/isohdpfx.bin
ISOLINUX += root/isolinux/isohdppx.bin
ISOLINUX += root/isolinux/isolinux.bin
ISOLINUX += root/isolinux/ldlinux.c32
ISOLINUX += root/isolinux/reboot.c32
ISOLINUX += root/isolinux/poweroff.c32
ISOLINUX += root/isolinux/ls.c32
ISOLINUX += root/isolinux/cat.c32

ISOLINUX += root/EFI/BOOT/ldlinux.e64
ISOLINUX += root/EFI/BOOT/syslinux.c32

# ISOLINUX += root/bin/$(BINFILE).uefi
ISOLINUX += root/bin/$(BINFILE)
ISOLINUX += root/isolinux/isolinux.cfg

root/isolinux/isolinux.cfg: root/bin/$(BINFILE) mk/boot.mk
	echo "label        $(BINFILE)" >  $@
	echo "default      $(BINFILE)" >> $@
	echo "kernel  /bin/$(BINFILE)" >> $@

.PHONY: isolinux
isolinux: bin/$(BINFILE).iso

.PHONY: iso
iso: bin/$(BINFILE).iso
bin/$(BINFILE).iso: $(ISOLINUX) mk/boot.mk
	xorriso -as mkisofs -o $@ -r root -V $(BINFILE) \
	-isohybrid-mbr root/isolinux/isohdpfx.bin -b isolinux/isolinux.bin \
	-c isolinux/boot.cat -boot-load-size 4 -boot-info-table \
	-no-emul-boot -isohybrid-gpt-basdat -isohybrid-apm-hfsplus

root/isolinux/%: /usr/lib/ISOLINUX/%
	cp $< $@
root/isolinux/%: /usr/lib/syslinux/modules/bios/%
	cp $< $@
root/EFI/BOOT/%: /usr/lib/syslinux/modules/efi64/%
	cp $< $@
root/bin/$(BINFILE).uefi: target/x86_64-unknown-uefi/debug/$(MODULE)
	cp $< $@
root/bin/$(BINFILE): target/x86_64-unknown-linux-gnu/debug/$(MODULE)
	cp $< $@
