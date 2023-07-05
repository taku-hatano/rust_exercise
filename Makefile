build: build_cargo build_python build_nodejs

build_cargo:
	cargo build

build_python:
	maturin build -i python3 --release --features python

build_nodejs:
	wasm-pack build --release --target nodejs --features nodejs
