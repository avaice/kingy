#!/bin/bash
set -e

cd "$(dirname "$0")"
mkdir -p build

nasm -f bin bootloader/boot.asm -o build/boot.bin
nasm -f bin bootloader/stage2.asm -o build/stage2.bin
nasm -f bin bootloader/kernel_test.asm -o build/kernel.bin

dd if=/dev/zero of=build/disk.img bs=512 count=2880 2>/dev/null            # 1.44MBの空フロッピー
dd if=build/boot.bin of=build/disk.img bs=512 seek=0 conv=notrunc 2>/dev/null   # セクタ1
dd if=build/stage2.bin of=build/disk.img bs=512 seek=1 conv=notrunc 2>/dev/null # セクタ2
dd if=build/kernel.bin of=build/disk.img bs=512 seek=2 conv=notrunc 2>/dev/null # セクタ3~

qemu-system-x86_64 -drive file=build/disk.img,format=raw,if=floppy -boot order=a -display cocoa,zoom-to-fit=on -monitor stdio
