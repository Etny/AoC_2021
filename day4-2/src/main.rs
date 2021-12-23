mod input;
mod board;

use input::*;
use board::*;

fn main() {

    let mut board_lines = INPUT_BOARDS.trim().lines().filter(|f| f.trim().len() > 0).peekable();

    let mut boards = Vec::new();
   
    while let Some(_) = board_lines.peek() {
        boards.push(Board::new(board_lines.by_ref().take(5)));
    }

    for num_str in INPUT_DRAWS.split(',') {
        let num = num_str.parse::<u32>().unwrap();

        let last_board = boards.len() == 1;

        for board in &mut boards {
            if let Some(score) = board.mark(num) {
                if last_board {
                    println!("{}", score);
                    return;
                }
            }
        }

        boards = boards.into_iter().filter(|b| !b.has_won()).collect();

    }

}
