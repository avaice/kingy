ENTRY(_start)

SECTIONS {
    /* アプリの番地の始まり（カーネルの paging::USER_BASE と同じ値） */
    . = 0x8000000000;

    .text : {                          /* 機械語。_start を必ず先頭に置く */
        KEEP(*(.text._start))
        *(.text .text.*)
    }
    .rodata : { *(.rodata .rodata.*) } /* 読み取り専用データ */
    .data : { *(.data .data.*) *(.got .got.*) } /* 初期値のある変数 */
    .bss : { *(.bss .bss.*) *(COMMON) } /* 初期値が0の変数（カーネルが0のページを渡すので、そのまま0） */

    /* カーネルが用意するアプリ用の領域（64KB）に収まっているか確かめる */
    ASSERT(. - 0x8000000000 <= 0x10000, "app is too large (max 64KB)")

    /DISCARD/ : { *(.eh_frame*) *(.note*) *(.comment*) }
}
