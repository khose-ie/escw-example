all: platform

APP_PATH=../target/thumbv7em-none-eabihf/debug/libapp.a

platform: $(APP_PATH)
	cargo build --package escw-example
	make -C escw-example-stm32 APP_PATH=../$(APP_PATH)

clean:
	cargo clean
	make -C escw-example-stm32 clean
