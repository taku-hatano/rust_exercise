build: build_cargo build_python build_nodejs

build_cargo:
	cargo build --no-default-features

build_python:
	maturin build -i python3 --release --features python --no-default-features

build_nodejs:
	wasm-pack build --release --target bundler --features nodejs --no-default-features
