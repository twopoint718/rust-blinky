app.hex: target/thumbv6m-none-eabi/release/app
	arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/release/app app.hex

target/thumbv6m-none-eabi/release/app: src/main.rs
	cargo build --release

clean:
	cargo clean
