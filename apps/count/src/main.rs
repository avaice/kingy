#![no_std]
#![no_main]

use kingy::{exit, write};

// 3, 2, 1 と数えて、終了コード 42 で終わるアプリ
#[unsafe(no_mangle)]
fn main() {
    for n in (1..=3).rev() {
        write(&[b'0' + n]);
        wait();
    }
    exit(42);
}

// 少し待つ（数を数えるだけ）
fn wait() {
    for i in 0..30_000_000 {
        core::hint::black_box(i); // コンパイラに「何もしないループ」として消されないようにする
    }
}
