
# Use this script if you want to use the arduino cli. Make sure ARM cross compiler is installed.
# Or just use `cargo embed --release` from the probe.rs team

cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/arduino-r4-wifi-template ./image.bin
arduino-cli upload -b arduino:renesas_uno:unor4wifi -i ./image.bin -p /dev/ttyACM0
