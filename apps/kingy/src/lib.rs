#![no_std]

use core::panic::PanicInfo;

const WRITE: u64 = 1;
const EXIT: u64 = u64::MAX - 1;

// 画面に文字を出す。成功したら 0、失敗したら u64::MAX
pub fn write(text: &[u8]) -> u64 {
    let result: u64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            inlateout("rax") WRITE => result, // 番号を渡して、結果を受け取る
            in("rdi") text.as_ptr(),          // 引数1：文字列の番地
            in("rsi") text.len(),             // 引数2：文字列の長さ
        );
    }
    result
}

// アプリを終わらせる。code はカーネルに伝える終了コード（0 = 成功）
pub fn exit(code: u64) -> ! {
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") EXIT,
            in("rdi") code,
            options(noreturn), // カーネルから戻ってこない
        );
        
        // カーネルが exit を受け付けずに戻ってきたら、自暴自棄になってエラーを起こす
        core::arch::asm!("ud2", options(noreturn));
    }
}

// アプリ側で書く main（各アプリの main.rs にある）
unsafe extern "Rust" {
    fn main();
}

// アプリの入口。カーネルはここ（USER_BASE）から実行を始める
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe { main() };
    exit(0); // main が最後まで来たら、終了コード 0 で終わる
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(101); // パニックしたら、終了コード 101 で終わる
}
