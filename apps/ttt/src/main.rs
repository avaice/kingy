#![no_std]
#![no_main]

use kingy::{write, exit, rect, get_key_code, draw_kbmp, print, pause};

static MARU_BMP: &[u8] = include_bytes!("../assets/maru.kbmp");
static BATSU_BMP: &[u8] = include_bytes!("../assets/batsu.kbmp");

#[unsafe(no_mangle)]
fn main() {

    let mut x = 0;
    let mut y = 0;
    let mut turn = 0;
    let mut board = [0; 9];

    write(b"Tic Tac Toe");

    draw_board(x, y, board);

    loop {
        if let Some(key) = get_key_code() {
            match key {
                0x48 if y > 0 => y -= 1,
                0x50 if y < 2 => y += 1,
                0x4B if x > 0 => x -= 1,
                0x4D if x < 2 => x += 1,
                0x1C => {
                    if board[x * 3 + y] != 0 {
                        continue;
                    }
                    board[x * 3 + y] = turn % 2 + 1;

                    turn += 1;

                    let winner = judge(board);
                    if winner == 0 {

                        if turn == 9 {
                            print(b"Draw!", 10, 100, 15);
                            draw_board(x, y, board);
                            pause();
                        }

                        loop{
                            let mut seed = rdtsc() | 1;
                            let cpu_x = xorshift(&mut seed) % 3;
                            let cpu_y = xorshift(&mut seed) % 3;
                            if board[cpu_x as usize * 3 + cpu_y as usize] != 0 {
                                continue;
                            }
                            board[cpu_x as usize * 3 + cpu_y as usize] = 2;
                            break;
                        }
                    }

                    let winner = judge(board);
                    if winner != 0 {
                        if winner == 1 {
                            print(b"You win!", 10, 100, 15);
                        } else {
                            print(b"You lose!", 10, 100, 15);
                        }
                    }

                    turn += 1;
                }
                0x01 => exit(0),
                _ => continue,
            }
            
            draw_board(x, y, board);
            

            let winner = judge(board);
            if winner != 0 {
                pause();
            }
        }
    }

}

fn judge(board: [usize; 9]) -> usize {
    for i in 0..3 {
        if board[i * 3] == board[i * 3 + 1] && board[i * 3 + 1] == board[i * 3 + 2] {
            return board[i * 3];
        }
    }
    for i in 0..3 {
        if board[i] == board[i + 3] && board[i + 3] == board[i + 6] {
            return board[i];
        }
    }
    if board[0] == board[4] && board[4] == board[8] {
        return board[0];
    }
    if board[2] == board[4] && board[4] == board[6] {
        return board[2];
    }
    return 0;
}

fn draw_board(x: usize, y: usize,  board: [usize; 9]) {
    for i in 0..3 {
        for j in 0..3 {
            let cursor = x == i && y == j;
            let board_type = board[i * 3 + j];
            draw_cell(i, j, board_type, cursor);
        }
    }
}

fn draw_cell(x: usize, y: usize, board_type: usize, cursor: bool) {
    if cursor {
        rect(170 + x as u64 * 100, 80 + y as u64 * 100, 100, 100, 7);
    }else{
        rect(170 + x as u64 * 100, 80 + y as u64 * 100, 100, 100, 15);
    }
    if board_type == 0 {
        return;
    }
    let kbmp = if board_type == 1 {
        MARU_BMP
    } else {
        BATSU_BMP
    };
    draw_kbmp(kbmp, 170 + x as u64 * 100, 80 + y as u64 * 100, 6, true);
}


fn rdtsc() -> u64 {
    let lo: u32;
    let hi: u32;
    unsafe { core::arch::asm!("rdtsc", out("eax") lo, out("edx") hi) };
    ((hi as u64) << 32) | lo as u64
}
fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}