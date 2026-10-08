ENTRY(_start)

SECTIONS {
    /* カーネルのアドレス始点 (stage2で指定した値) */
    . = 0x20000;

    .text : { /* 機械語 */
        KEEP(*(.text._start))
        *(.text .text.*)
    }
    .rodata : { *(.rodata .rodata.*) } /* 読み取り専用データ */
    .data.  : { *(.data .data.*) }     /* 初期値のある変数 */
    .bss    : {                        /* 初期値が0の変数 */
        __bss_start = .;
        *(.bss .bss.*) *(COMMON)
        __bss_end = .;
    }

    /* いらない情報 */
    /DISCARD/ : { *(.eh_frame*) *(.note*) *(.comment*) }
}