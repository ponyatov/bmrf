APT += gcc-i686-linux-gnu syslinux isolinux xorriso

QEMU      = qemu-system-$(ARCH)

ISOLINUX += root/isolinux/isohdpfx.bin
ISOLINUX += root/isolinux/isohdppx.bin
ISOLINUX += root/isolinux/isolinux.bin
ISOLINUX += root/isolinux/ldlinux.c32
ISOLINUX += root/isolinux/reboot.c32
ISOLINUX += root/isolinux/poweroff.c32
ISOLINUX += root/isolinux/ls.c32
ISOLINUX += root/isolinux/cat.c32
ISOLINUX += root/isolinux/libcom32.c32
ISOLINUX += root/isolinux/reboot.c32

ISOLINUX += root/EFI/BOOT/ldlinux.e64
ISOLINUX += root/EFI/BOOT/syslinux.c32

# ISOLINUX += root/bin/$(BINFILE).uefi
ISOLINUX += root/isolinux/isolinux.cfg

BINS += root/bin/$(BINFILE).i486.bare
BINS += root/bin/$(BINFILE).x86_64.linux
# BINS += root/bin/$(BINFILE).cortexM4.elf
# BINS += root/bin/$(BINFILE).i686.efi

root/isolinux/isolinux.cfg: $(BINS)
	echo "label        $(BINFILE).i486.bare" >  $@
	echo "default      $(BINFILE).i486.bare" >> $@
	echo "kernel  /bin/$(BINFILE).i486.bare" >> $@

.PHONY: iso
iso: bin/$(BINFILE).iso

.PHONY: qemu
qemu: bin/$(BINFILE).iso
	$(QEMU) -boot d -cdrom $<

bin/$(BINFILE).iso: $(ISOLINUX)
	xorriso -as mkisofs -o $@ -r root -V $(MODULE)@$(HW) \
	-isohybrid-mbr root/isolinux/isohdpfx.bin -b isolinux/isolinux.bin \
	-c isolinux/boot.cat -boot-load-size 4 -boot-info-table \
	-no-emul-boot -isohybrid-gpt-basdat -isohybrid-apm-hfsplus

root/isolinux/%: /usr/lib/ISOLINUX/%
	cp $< $@
root/isolinux/%: /usr/lib/syslinux/modules/bios/%
	cp $< $@
root/EFI/BOOT/%: /usr/lib/syslinux/modules/efi64/%
	cp $< $@

root/bin/$(BINFILE).x86_64.linux: target/x86_64-unknown-linux-gnu/debug/$(MODULE)
	rm -f $(dir $@)*.x86_64.linux ; cp $< $@
root/bin/$(BINFILE).i686.efi: target/i686-unknown-uefi/debug/$(MODULE).efi
	rm -f $(dir $@)*.i686.efi ; cp $< $@
root/bin/$(BINFILE).i486.bare: target/i486-pc-bare/debug/$(MODULE).i486.bare
	rm -f $(dir $@)*.i486.bare ; cp $< $@
root/bin/$(BINFILE).cortexM4.elf: target/

target/x86_64-unknown-linux-gnu/debug/$(MODULE):
	cargo build --features=pc --target=x86_64-unknown-linux-gnu
target/i486-pc-bare/debug/$(MODULE).i486.bare:
	cargo +nightly build -Zbuild-std=core,alloc,compiler_builtins -Zbuild-std-features=compiler-builtins-mem --target .cargo/i486-pc-bare.json --features qemu386
target/i686-unknown-uefi/debug/$(MODULE).efi:
	cargo build --target i686-unknown-uefi --features i686,uefi

.PHONY: multiboot
multiboot: tmp/multiboot2.elf

tmp/%.elf: arch/x86/src/%.s arch/x86/inc/%.h arch/x86/x86.ld arch/x86/x86.mk
	i686-linux-gnu-gcc -ffreestanding -nostdlib -I $(dir $<)  -o $@.o -c $<
	i686-linux-gnu-gcc -ffreestanding -nostdlib -T arch/x86/x86.ld -o $@ $@.o
	i686-linux-gnu-objdump -xd $@
	$(QEMU) $(QEMU_CFG) -kernel $@
