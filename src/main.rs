use std::io::{self, Write};

fn print_board(board: &mut [[char; 8]; 8]){
    print!("  ");
    for i in 0..8 {
        print!("{}", (b'a' + i as u8) as char);
    }
    println!();

    // The center four coordinates are placed with colors in the initial board.
    board[3][3] = 'W';
    board[3][4] = 'B';
    board[4][3] = 'B';
    board[4][4] = 'W';

    // Print the rest of the board.
    for j in 0..8 {
        // Print the coordinate reference for rows.
        print!("{} ", (b'a' + j as u8) as char);
        for k in 0..8 {
            print!("{}", board[j][k]);
        }
        println!();
    }
}

fn show_board_after_move(board: &mut [[char; 8]; 8]){
    print!("  ");
    for i in 0..8 {
        print!("{}", (b'a' + i as u8) as char);
    }
    println!();

    for j in 0..8 {
        // Print the coordinate reference for rows.
        print!("{} ", (b'a' + j as u8) as char);
        for k in 0..8 {
            print!("{}", board[j][k]);
        }
        println!();
    }
}

fn position_in_bounds(board: &mut [[char; 8]; 8], row: char, col: char) -> bool {
    // Convert row and col to indices based on 'a'.
    let row_idx: i32 = row as i32 - 'a' as i32;
    let col_idx: i32 = col as i32 - 'a' as i32;

    // Check if the row is out of bounds (north or south).
    if row_idx < 0 || row_idx >= 8 {
        return false;
    }

    // Check if the col is out of bounds (west or east).
    if col_idx < 0 || col_idx >= 8 {
        return false;
    }

    // The coordinate is within bounds.
    true
}

fn main() {
    //println!("Hello, world!");
    let mut board: [[char; 8]; 8] = [['.'; 8]; 8]; 
    print_board(&mut board);
}

