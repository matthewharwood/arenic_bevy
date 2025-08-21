# Arenic Build Commands

# Build and serve web version locally
web:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/arenic_bevy.wasm
	cp -r assets web/
	@echo "✅ Web build complete! Run 'make serve' to start the server"

# Serve the web build
serve:
	python3 -m http.server 8000 --directory web

# Build and run native version
run:
	cargo run --release

# Clean build artifacts
clean:
	cargo clean
	rm -rf web/assets web/*.js web/*.wasm

# Build for all platforms (local testing)
build-all: web
	cargo build --release

.PHONY: web serve run clean build-all