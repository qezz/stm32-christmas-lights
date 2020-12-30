build:
	cargo build --release
	cargo objcopy --bin stm32f1_leds --target thumbv7m-none-eabi --release -- -O binary stm32f1_leds.bin

flash: build
	sudo st-info --probe && sleep 2
	sudo st-flash erase && sleep 2
	sudo st-flash write stm32f1_leds.bin 0x8000000
