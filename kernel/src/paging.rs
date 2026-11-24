use crate::{graphics, memory};

const PRESENT: u64 = 1 << 0;  // この行は有効
const WRITABLE: u64 = 1 << 1; // 書き込みOK
const USER: u64 = 1 << 2;     // アプリ（Ring3）も入ってOK
const NO_CACHE: u64 = 1 << 4; // キャッシュしない（画面用）
const HUGE: u64 = 1 << 7;     // 2MBをまとめて1行で表す
const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000; // 行の中の「住所」部分

// アプリ用のアドレス空間の始まり
pub const USER_BASE: usize = 0x80_0000_0000;

static mut KERNEL_PML4: usize = 0;
static mut KERNEL_PDPT: usize = 0;

// アドレスを512行の表として扱う
// 1行は住所を入れるために 8バイト必要。4KB ÷ 8バイト = 512行
fn table(addr: usize) -> &'static mut [u64; 512] {
    unsafe { &mut *(addr as *mut [u64; 512]) }
}

// 次の段の表の住所を返す。まだなければページを借りて作る
fn next_table(entry: &mut u64, flags: u64) -> usize {
    if *entry & PRESENT == 0 {
        let page = memory::alloc().unwrap();
        *entry = page as u64 | flags;
    }
    (*entry & ADDR_MASK) as usize
}

// PD以下をまとめて登録する
fn map_huge(pdpt: usize, virt: usize, phys: usize, flags: u64) {
    let index = (virt >> 30) & 0x1FF;
    let entry = &mut table(pdpt)[index];
    let pd = next_table(entry, PRESENT | WRITABLE);
    table(pd)[(virt >> 21) & 0x1FF] = phys as u64 | flags | HUGE;
}

pub fn init() {
    let pml4 = memory::alloc().unwrap();
    let pdpt = memory::alloc().unwrap();
    table(pml4)[0] = pdpt as u64 | PRESENT | WRITABLE;

    map_huge(pdpt, 0, 0, PRESENT | WRITABLE);

    let fb = graphics::vram() & !0x1F_FFFF;
    map_huge(pdpt, fb, fb, PRESENT | WRITABLE | NO_CACHE);

    unsafe {
        KERNEL_PML4 = pml4;
        KERNEL_PDPT = pdpt;
    }
    switch(pml4);
}

// アプリ用のアドレス空間を新しく作る
pub fn new_address_space() -> usize {
    let pml4 = memory::alloc().unwrap();
    table(pml4)[0] = unsafe { KERNEL_PDPT } as u64 | PRESENT | WRITABLE; // USERなし = 立入禁止
    pml4
}

// アプリ用のページを登録する
pub fn map_user_page(pml4: usize, virt: usize, phys: usize) {
    if virt < USER_BASE {
        return; // カーネルの場所には登録させない
    }
    let flags = PRESENT | WRITABLE | USER;
    let pdpt = next_table(&mut table(pml4)[(virt >> 39) & 0x1FF], flags);
    let pd = next_table(&mut table(pdpt)[(virt >> 30) & 0x1FF], flags);
    let pt = next_table(&mut table(pd)[(virt >> 21) & 0x1FF], flags);
    table(pt)[(virt >> 12) & 0x1FF] = phys as u64 | flags;
}

// アドレス空間を切り替える
pub fn switch(pml4: usize) {
    unsafe { core::arch::asm!("mov cr3, {}", in(reg) pml4) };
}

// カーネルのアドレス空間を解放する
pub fn kernel_space() -> usize {
    unsafe { KERNEL_PML4 }
}

// アプリのアドレス空間と、そこに登録したページを全部解放する
pub fn free_address_space(pml4: usize) {
    for i in 1..512 { // 0行目はカーネルと共有しているので触らない
        let entry = table(pml4)[i];
        if entry & PRESENT != 0 {
            free_table((entry & ADDR_MASK) as usize, 3);
        }
    }
    memory::free(pml4);
}

// 表と、その下にあるものを全部解放する（level: PDPT = 3, PD = 2, PT = 1）
fn free_table(addr: usize, level: u32) {
    for i in 0..512 {
        let entry = table(addr)[i];
        if entry & PRESENT != 0 {
            let next = (entry & ADDR_MASK) as usize;
            if level == 1 {
                memory::free(next); // PT の行の先は、アプリのページそのもの
            } else {
                free_table(next, level - 1); // 1段下の表へ
            }
        }
    }
    memory::free(addr); // 中身を全部解放してから、表そのものを解放する
}