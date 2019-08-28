use read_input::prelude::input;
use read_input::InputBuild;

struct BoardCell {
    fixed: bool,
    val: u8
}

fn init(val: u8) -> BoardCell {
    BoardCell {
        fixed: match val {
            1..=9 => true,
            0 => false,
            _ => panic!("Invalid value: {}", val)
        },
        val
    }
}

fn main() {
    let board = &mut Vec::new();
    for i in 1..=9 {
        let mut values = read_row(i);
        while !valid_row(&values) {
            println!("Invalid row, try again");
            values = read_row(i);
        }
        values.iter().map(|v|  v.parse::<u8>().unwrap()).for_each(|i| board.push(init(i)));
    }

    run(board)
}

fn read_row(idx: u8) -> Vec<String> {
    let row: String = input().repeat_msg(format!("Row {}: ", idx)).get();
    row.split_whitespace().map(String::from).collect::<Vec<String>>()
}

fn valid_row(row: &[String]) -> bool {
    row.iter().filter(
        |v| v.parse::<u8>().is_ok() && (0..=9).contains(&v.parse::<u8>().unwrap())
    ).count() == 9
}

fn run(board: &mut[BoardCell]) {
    let mut idx = 0;
    while (0..81).contains(&idx) {
        if board[idx].fixed {
            idx += 1;
            continue;
        }

        let mut val = board[idx].val + 1;
        while !is_valid(&board, idx, val) {
            val += 1;
        }

        match val {
            1..=9 => {
                board[idx].val = val;
                idx += 1;
            }
            _ => {
                board[idx].val = 0;
                idx = backtrack(&board, idx);
            }
        };
    }

    print_board(&board);
}

fn backtrack(board: &[BoardCell], idx: usize) -> usize {
    for i in (0..idx).rev() {
        if !board[i].fixed { return i }
    }

    panic!("Unsolvable!");
}

fn is_valid(board: &[BoardCell], idx:usize, val:u8) -> bool {
    check_row(board, idx, val) && check_col(board, idx, val) && check_square(board, idx, val)
}

fn check_row(board: &[BoardCell], idx: usize, val: u8) -> bool {
    !board.iter().skip((idx / 9) * 9).take(9).map(|c| c.val).any(|v| v == val)
}

fn check_col(board: &[BoardCell], idx: usize, val: u8) -> bool {
    !board.iter().skip(idx % 9).step_by(9).map(|c| c.val).any(|v| v == val)
}

fn check_square(board: &[BoardCell], idx: usize, val: u8) -> bool {
    let start = idx - (((idx / 9) % 3) * 9) - (idx % 9 % 3);

    !board.iter().skip(start).take(3).chain(
        board.iter().skip(start + 9).take(3).chain(
            board.iter().skip(start + 18).take(3)
        )
    ).map(|c| c.val).any(|v| v == val)
}

fn print_board(board: &[BoardCell]) {
    for i in 0..9 {
        if i > 0 && i % 3 == 0 {
            println!("-----------------------------------");
        }

        board.iter().skip(i * 9).take(9).enumerate().for_each(|(i, c)| {
            if i % 9 == 0 { print!("{}", c.val) }
            else if i % 3 == 0 { print!(" || {}", c.val) }
            else { print!(" | {}", c.val) }
        });

        println!();
    }
}