.PHONY: build test install uninstall

build:
	cargo build --release

test:
	cargo test

install:
	cargo install --path . --locked

uninstall:
	cargo uninstall termgammon
