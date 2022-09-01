.PHONY: flash

flash:
	avrdude -patmega1284p -carduino -P/dev/ttyUSB0 -b57600 -D -Uflash:w:target/avr-unknown-gnu-atmega1284p/release/vroom.elf -q

shell:
	avrdude -patmega1284p -carduino -P/dev/ttyUSB0 -b57600 -D -t
