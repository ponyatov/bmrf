.PHONY: install update
install: rust
	$(MAKE) update
update:

.PHONY: rust
rust: $(RUSTUP)
	$< self update
$(RUSTUP):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
