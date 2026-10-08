bits 16
org 0x8000                                          ; stage2のメモリ位置

start:
    cli
    xor ax, ax
    mov ds, ax                                      ; データセグメント
    mov [boot_drive], dl                            ; boot.asmから渡したドライブ番号を保存
    sti

    ; 16セクタずつkernel 読み込み (3~18)
    mov ax, 0x2000
    mov es, ax                                      ; エクストラセグメント
    xor bx, bx                                      ; ES:BX = 0x2000:0 = 0x20000 になる
    mov ah, 0x02                                    ; 0x02 = ディスク読み込み
    mov al, 16                                      ; 読み込むセクタ数(FDなら1sector = 512byte)
    mov ch, 0                                       ; シリンダ番号
    mov cl, 3                                       ; セクタ番号
    mov dh, 0                                       ; ヘッド番号
    mov dl, [boot_drive]                            ; どのドライブから読み込むか
    int 0x13                                        ; BIOSのディスク機能を呼ぶ（失敗でCF=1）
    jc halt                                         ; CF=1ならhaltへ移動

    ; 16セクタずつkernel 読み込み (裏面 1~16)
    mov ax, 0x2200
    mov es, ax                                      ; エクストラセグメント
    xor bx, bx                                      ; ES:BX = 0x2200:0 = 0x22000 になる
    mov ah, 0x02                                    ; 0x02 = ディスク読み込み
    mov al, 16                                      ; 読み込むセクタ数(FDなら1sector = 512byte)
    mov ch, 0                                       ; シリンダ番号
    mov cl, 1                                       ; セクタ番号
    mov dh, 1                                       ; ヘッド番号（表裏）
    mov dl, [boot_drive]                            ; どのドライブから読み込むか
    int 0x13                                        ; BIOSのディスク機能を呼ぶ（失敗でCF=1）
    jc halt                                         ; CF=1ならhaltへ移動

    xor ax, ax
    mov es, ax
    mov di, 0x9000                                  ; どこにモード情報を書き込むか
    mov ax, 0x4F01                                  ; モード情報の取得を行う
    mov cx, 0x101                                   ; カウントレジスタ。0x101はVBEのモード番号
    int 0x10                                        ; 画面モード取得 成功したらax = 0x004F
    cmp ax, 0x004F                                  ; axの値が0x004Fか？ZFフラグが立つ
    jne halt                                        ; ZF=0ならhalt
    mov eax, [0x9000 + 0x28]                        ; 取得したモード情報からフレームバッファの物理アドレスを取得
    mov [fb_addr], eax                              ; フレームバッファのアドレスをfb_addrに格納する

    mov ax, 0x4F02                                  ; VBEのモード設定
    ; リニアフレームバッファ: 画面全体が1本の連続したメモリとして見える。
    ; 0x9028 から読んだアドレス（PhysBasePtr）+ y × 640 + x に1バイト書けば、その位置の点に色が付く
    mov bx, 0x4101                                  ; 0x101 + リニアフレームバッファ
    int 0x10
    cmp ax, 0x004F
    jne halt

    cli
    lgdt [gdt_descriptor]                           ; GDTの場所と大きさをCPUに伝える
    mov eax, cr0                                    ; cr0の値を取得する
    or eax, 1                                       ; eaxの最下位ビットを1にする（最下位ビットはプロテクトモードの判定）
    mov cr0, eax                                    ; 変更したeaxをcr0に入れる
    jmp dword 0x08:protected_mode                   ; csに0x08を入れて32bitモードに切り替える

halt:
    cli
.loop:
    hlt                                             ; CPUを停止
    jmp .loop                                       ; 無限ループ

fb_addr equ 0x600                                   ; equは数値に名前をつける
boot_drive db 0

gdt_start:
    dq 0
    dq 0x00CF9A000000FFFF                           ; 0x08
    dq 0x00CF92000000FFFF                           ; 0x10 (16進数なので8byte進んだ状態)
    dq 0x00AF9A000000FFFF                           ; 0x18
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

bits 32
protected_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov esp, 0x7C00

    mov edi, 0x10000
    mov ecx, 4096
    xor eax, eax
    cld
    
    ; stosd = 4byte書き込み
    rep stosd                                       ; ediが指すアドレスから16KB分 eaxの値(0)で埋める

    mov dword [0x10000], 0x11003
    mov dword [0x11000], 0x12003
    mov dword [0x12000], 0x83

    mov eax, [fb_addr]
    mov ebx, eax
    shr ebx, 30                                     ; ビットを右に30bitずらす
    mov dword [0x11000 + ebx*8], 0x13003
    mov ebx, eax
    shr ebx, 21
    and ebx, 0x1FF
    and eax, 0xFFE00000
    or eax, 0x93
    mov [0x13000 + ebx*8], eax
    add eax, 0x200000
    mov [0x13008 + ebx*8], eax

    mov eax, 0x10000
    mov cr3, eax

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    jmp 0x18:long_mode

bits 64
long_mode:
    mov rsp, 0x7C00
    jmp 0x20000


times 512-($-$$) db 0