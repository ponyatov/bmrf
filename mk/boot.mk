ISOLINUX += root/isolinux/isohdpfx.bin
ISOLINUX += root/isolinux/isohdppx.bin
ISOLINUX += root/isolinux/isolinux.bin

ISOLINUX += root/EFI/BOOT/ldlinux.e64
ISOLINUX += root/EFI/BOOT/syslinux.c32

.PHONY: isolinux
isolinux: $(ISOLINUX)

root/isolinux/%: /usr/lib/ISOLINUX/%
	cp $< $@
root/EFI/BOOT/%: /usr/lib/syslinux/modules/efi64/%
	cp $< $@
