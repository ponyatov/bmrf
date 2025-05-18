.PHONY: i486
i486:
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
	cargo +nightly build -Zbuild-std --target .cargo/i486-pc-bare.json
