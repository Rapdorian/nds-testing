bin=test-arm9
release = 

all: bin/$(bin).nds

run: release
	desmume-cli bin/$(bin).nds

debug: bin/$(bin.nds)
	desmume-cli --arm9gdb 2000 bin/$(bin).nds &
	RUST_GDB=arm-none-eabi-gdb rust-gdb target/armv5te-none-eabi/debug/${bin}

release: release = --release
release: bin/$(bin).nds

obj:
	mkdir obj
bin: 
	mkdir bin

bin/$(bin).nds: obj/$(bin).arm9 obj/$(bin).arm7 bin
	ndstool -c bin/$(bin).nds -9 obj/$(bin).arm9 -7 obj/$(bin).arm7

obj/$(bin).arm9: obj
	RUST_TARGET_PATH=${PWD} xargo objcopy --bin $(bin) $(release) -- -O binary $@

obj/$(bin).arm7: obj/$(bin).arm7.elf
	arm-none-eabi-objcopy -O binary $< $@

obj/$(bin).arm7.elf: arm7/main.s obj
	arm-none-eabi-as $< -o $@
