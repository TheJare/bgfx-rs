rust-bindgen:
	git clone https://github.com/servo/rust-bindgen

rust-bindgen/target/release/bindgen: rust-bindgen
	cd rust-bindgen
	cd rust-bindgen && cargo build --release

src/ffi_platform.rs: rust-bindgen/target/release/bindgen bgfx/include/bgfx/c99/bgfx.h bgfx/include/bgfx/c99/platform.h
	rust-bindgen/target/release/bindgen \
		--no-prepend-enum-name \
		--constified-enum ".*" \
		-o src/ffi_platform.rs \
		bgfx/include/bgfx/c99/platform.h -- \
		-I bgfx/include \
		-I bx/include