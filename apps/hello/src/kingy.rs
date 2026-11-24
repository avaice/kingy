const WRITE: u64 = 1;

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
