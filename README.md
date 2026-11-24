# Kingy

自作OSを、、作るぞ

<img width="613" height="490" alt="image" src="https://github.com/user-attachments/assets/0cd563cc-a610-423a-b432-635b809be381" />

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
