# Kingy

自作OSを、、作るぞ

<img width="７７９" height="587" alt="718803380-7ce48c3a-f5cc-4a49-adb6-b73233d1d874" src="https://github.com/user-attachments/assets/12f7b088-0a3a-46af-820e-2daddba1c705" />

## requires
```bash
brew install nasm qemu

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add x86_64-unknown-none
rustup component add llvm-tools
```

## build & run
```bash
./dev.sh
```
