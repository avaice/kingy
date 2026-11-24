#!/bin/bash
set -e

OBJCOPY="$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/host: //p')/bin/rust-objcopy"

cd "$(dirname "$0")"
mkdir -p build

# アプリをビルドする（カーネルが build/<名前>.bin を埋め込むので、カーネルより先にやる）
(cd apps && cargo build --release)
for main in apps/*/src/main.rs; do  # src/main.rs があるものがアプリ
    name="$(basename "$(dirname "$(dirname "$main")")")"
    "$OBJCOPY" -O binary "apps/target/x86_64-unknown-none/release/$name" "build/$name.bin"
done

cd kernel
cargo build --release
"$OBJCOPY" -O binary target/x86_64-unknown-none/release/kernel ../build/kernel.bin
cd ..

kernel_size=$(wc -c < build/kernel.bin)
if [ "$kernel_size" -gt 16384 ]; then
    echo "error: build/kernel.bin が ${kernel_size} バイトあり、stage2 が読み込める 16384 バイトを超えています" >&2
    exit 1
fi

nasm -f bin bootloader/boot.asm -o build/boot.bin
nasm -f bin bootloader/stage2.asm -o build/stage2.bin

dd if=/dev/zero of=build/disk.img bs=512 count=2880 2>/dev/null            # 1.44MBの空フロッピー
dd if=build/boot.bin of=build/disk.img bs=512 seek=0 conv=notrunc 2>/dev/null   # セクタ1
dd if=build/stage2.bin of=build/disk.img bs=512 seek=1 conv=notrunc 2>/dev/null # セクタ2
dd if=build/kernel.bin of=build/disk.img bs=512 seek=2 conv=notrunc 2>/dev/null # セクタ3~

qemu-system-x86_64 -drive file=build/disk.img,format=raw,if=floppy -boot order=a -display cocoa,zoom-to-fit=on -monitor stdio
