cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/virtualized-hardware-learning ./image.bin
arduino-cli upload -b arduino:renesas_uno:unor4wifi -i ./image.bin -p /dev/ttyACM0
