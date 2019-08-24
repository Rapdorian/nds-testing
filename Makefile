all: main.nds

run: main.nds
	desmume main.nds

obj:
	mkdir obj

main.nds: obj/main.arm9 obj/main.arm7
	ndstool -c main.nds -9 obj/main.arm9 -7 obj/main.arm7

obj/main.arm9: obj
	xargo objcopy --bin test-arm9 --release -- -O binary $@

obj/main.arm7: obj/main.arm7.elf
	arm-none-eabi-objcopy -O binary $< $@

obj/main.arm7.elf: arm7/main.s obj
	arm-none-eabi-as $< -o $@
