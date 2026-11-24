// セグメントの番号（GDTの何バイト目か）
pub const KERNEL_CODE: u16 = 0x08;
pub const KERNEL_DATA: u16 = 0x10;
pub const USER_DATA: u16 = 0x18;
pub const USER_CODE: u16 = 0x20;
const TSS_SELECTOR: u16 = 0x28;

// Ring3 から Ring0 に入るときに使うスタックの場所などを書いておく表
#[repr(C, packed)]
struct Tss {
    reserved0: u32,
    rsp: [u64; 3],  // rsp[0] = Ring0に入るときに使うスタック
    reserved1: u64,
    ist: [u64; 7],  // 非常用スタック
    reserved2: u64,
    reserved3: u16,
    iomap_base: u16,
}

static mut TSS: Tss = Tss {
    reserved0: 0, rsp: [0; 3], reserved1: 0, ist: [0; 7],
    reserved2: 0, reserved3: 0, iomap_base: 0,
};

// アプリの実行中に割り込みが来たとき、カーネルが使うスタック（16KB）
static mut KERNEL_STACK: [u8; 16384] = [0; 16384];

// GDT本体。TSSの行は16バイトなので2行ぶん使う
static mut GDT: [u64; 7] = [
    0,                     // 0x00: 空（必須）
    0x00AF_9A00_0000_FFFF, // 0x08: カーネルのコード（Ring0, 64bit）
    0x00CF_9200_0000_FFFF, // 0x10: カーネルのデータ（Ring0）
    0x00CF_F200_0000_FFFF, // 0x18: アプリのデータ（Ring3）
    0x00AF_FA00_0000_FFFF, // 0x20: アプリのコード（Ring3, 64bit）
    0,                     // 0x28: TSS（前半）init() で埋める
    0,                     //       TSS（後半）
];

#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u64,
}

pub fn init() {

    // TODO: ここのコードはマジでわからなかったので、AIが教えてくれたままです。要確認
    unsafe {
        // Ring0に入るときに使うスタックをTSSに指定する
        let stack_top = ((&raw const KERNEL_STACK) as u64 + 16384) & !0xF; // 16の倍数にそろえる
        TSS.rsp[0] = stack_top;

        // TSS の場所と大きさをGDTに指定する
        let base = (&raw const TSS) as u64;
        let limit = (core::mem::size_of::<Tss>() - 1) as u64;
        GDT[5] = (limit & 0xFFFF)
            | ((base & 0xFF_FFFF) << 16)
            | (0x89 << 40)                  // 有効なTSS（64bit）
            | (((limit >> 16) & 0xF) << 48)
            | (((base >> 24) & 0xFF) << 56);
        GDT[6] = base >> 32;

        let pointer = GdtPointer {
            limit: (core::mem::size_of::<[u64; 7]>() - 1) as u16,
            base: (&raw const GDT) as u64,
        };
        core::arch::asm!("lgdt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags));

        // CS（コードの番号）を新しいGDTの 0x08 に入れ直す。CSは mov で直接書けないので、
        // 「戻り先」を積んで retfq（遠くへの return）で入れ替える
        core::arch::asm!(
            "push {sel}",
            "lea {tmp}, [rip + 2f]",
            "push {tmp}",
            "retfq",
            "2:",
            sel = in(reg) KERNEL_CODE as u64,
            tmp = lateout(reg) _,
        );

        // データ用の番号も新しいGDTの 0x10 に入れ直す
        core::arch::asm!(
            "mov ds, {0:x}",
            "mov es, {0:x}",
            "mov ss, {0:x}",
            in(reg) KERNEL_DATA,
        );

        // TSS を読み込む
        core::arch::asm!("ltr {0:x}", in(reg) TSS_SELECTOR);
    }
}