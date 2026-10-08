bits 16                                             ; 16bit modeにする
org 0x7c00                                          ; このプログラムがどこに置かれる予定か

start:
    jmp 0x0000:main                                 ; 0x0000をCSに設定して、main番地先へ移動する（mainと打つと、mainのアドレスになる）

main:
    cli                                             ; 割り込み禁止
    xor ax, ax                                      ; ax = 0 にする（axは汎用レジスタ）
    mov ds, ax                                      ; データセグメント
    mov ss, ax                                      ; スタックセグメント
    mov sp, 0x7c00                                  ; スタックの終端は0x7c00（ここから0x0000の方向に向かってstackされていく）

    mov [boot_drive], dl                            ; dl（起動ドライブの番号が入っている）の値をboot_driveに格納

    sti                                             ; 割り込みOK
    mov ax, 0x0003                                  ; 80 x 25 テキストモード
    int 0x10                                        ; 画面モード設定
    cli

    mov ax, 0xB800                                  ; axにVGAテキストメモリの先頭を渡す（esには直接定義できないため）
    mov es, ax                                      ; esにVGAテキストメモリの先頭を渡す

    mov si, message                                 ; messageの内容を読み出す
    xor di, di                                      ; di = 0
    cld                                             ; 方向フラグを++側にする。lodsbで前から読んでくれるようになる

.print:
    lodsb                                           ; siを1byte読んでalに入れる。siは1byte進む
    test al, al                                     ; al AND al を計算してRFLAGSのZF（直前の計算結果が0か？）に格納する
    jz .load_stage2                                 ; ZF = 1ならload_stage2に飛ぶ

    mov [es:di], al                                 ; 文字を表示
    mov byte [es:di+1], 0x0F                        ; 文字色・背景色の設定
    add di, 2                                       ; 次の文字の位置へ（文字コード＋色指定で2byteなので）
    jmp .print                                      ; .printの先頭に移動

.load_stage2:
    xor ax, ax
    mov es, ax                                      ; esを0にする
    mov bx, 0x8000                                  ; stage2のデータをコピーする先のアドレス

    mov ah, 0x02                                    ; 0x02 = ディスク読み込み
    mov al, 1                                       ; 読み込むセクタ数(FDなら1sector = 512byte)
    mov ch, 0                                       ; シリンダ番号
    mov cl, 2                                       ; セクタ番号
    mov dh, 0                                       ; ヘッド番号
    mov dl, [boot_drive]                            ; どのドライブから読み込むか

    sti
    int 0x13                                        ; BIOSのディスク機能を呼ぶ（失敗でCF=1）
    jc .halt                                        ; CF=1なら.haltへ移動

    mov dl, [boot_drive]                            ; 念のため、stage2にジャンプする前にもういちど読み込んでおく
    jmp 0x8000                                      ; stage2に移動

.halt:
    hlt                                             ; CPUを停止
    jmp .halt                                       ; 無限ループ

message db 'booting KingyOS...', 0                  ; messageに文字列をbyte列で並べる
boot_drive db 0

times 510-($-$$) db 0                               ; timesは繰り返し命令。510-(現在位置-先頭位置) 回 0を書き込む

dw 0xAA55                                           ; ブートシグネチャの0x55 0xAAを書き込む（リトルエンディアンなので逆）