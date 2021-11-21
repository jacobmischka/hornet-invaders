CRATE_NAME=hornet-invaders

.PHONY: build web

web: build
	cp target/wasm32-unknown-unknown/release/${CRATE_NAME}.wasm web/

build:
	cargo build --release --target wasm32-unknown-unknown

