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

fn legal_move_exists(board: &mut [[char; 8]; 8], colour: char) -> bool {
    let mut row: char;
    let mut col: char;

    // Loop through the board
    for i in 0..8 {
        for j in 0..8 {
            // Check if the current cell is unoccupied
            if board[i][j] == '.' {
                // Convert array indices to board coordinates
                row = ('a' as u8 + i as u8) as char;
                col = ('a' as u8 + j as u8) as char;

                // Check legal moves in all eight directions
                if check_legal_in_direction(board, row, col, colour, -1, -1)
                    || check_legal_in_direction(board, row, col, colour, -1, 0)
                    || check_legal_in_direction(board, row, col, colour, -1, 1)
                    || check_legal_in_direction(board, row, col, colour, 0, -1)
                    || check_legal_in_direction(board, row, col, colour, 0, 1)
                    || check_legal_in_direction(board, row, col, colour, 1, -1)
                    || check_legal_in_direction(board, row, col, colour, 1, 0)
                    || check_legal_in_direction(board, row, col, colour, 1, 1)
                {
                    return true;
                }
            }
        }
    }
    // No legal moves found
    false
}

// Check if the move is valid
fn check_valid_move(board: &mut [[char; 8]; 8], colour: char, row: char, col: char) -> bool {
    // Convert row and col to indices
    let row_idx = row as usize - 'a' as usize;
    let col_idx = col as usize - 'a' as usize;

    // The move is placed on a pre-occupied coordinate, so it's illegal
    if board[row_idx][col_idx] != '.' {
        return false;
    }

    // Check all 8 directions
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for &(delta_row, delta_col) in directions.iter() {
        if check_legal_in_direction(board, row, col, colour, delta_row, delta_col) {
            return true;
        }
    }

    // If none of the directions are legal, the move is illegal
    false
}

// Perform the move and flip the colors in the necessary directions
fn perform_move(board: &mut [[char; 8]; 8], colour: char, row: char, col: char) {
    // List of all 8 directions
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for &(delta_row, delta_col) in directions.iter() {
        if check_legal_in_direction(board, row, col, colour, delta_row, delta_col) {
            flip_colour(board,  row, col, colour, delta_row, delta_col);
        }
    }
}

// Flip the color in the specified direction
fn flip_colour(board: &mut [[char; 8]; 8], mut row: char, mut col: char,
               colour: char, delta_row: isize, delta_col: isize) {
    // Convert row and col to indices and place the color on the coordinate
    let mut row_idx = row as usize - 'a' as usize;
    let mut col_idx = col as usize - 'a' as usize;
    board[row_idx][col_idx] = colour;

    // Move to the next coordinate in the specified direction
    row = ((row as isize + delta_row) as u8) as char;
    col = ((col as isize + delta_col) as u8) as char;

    // Keep flipping the color in the direction until we encounter the same color
    while board[row as usize - 'a' as usize][col as usize - 'a' as usize] != colour {
        row_idx = row as usize - 'a' as usize;
        col_idx = col as usize - 'a' as usize;
        board[row_idx][col_idx] = colour;

        row = ((row as isize + delta_row) as u8) as char;
        col = ((col as isize + delta_col) as u8) as char;
    }
}

// Check if there is a legal move in a specific direction
fn check_legal_in_direction(board: &mut [[char; 8]; 8], mut row: char, mut col: char,
                            colour: char, delta_row: isize, delta_col: isize) -> bool {
    // Move to the next coordinate in the specified direction
    row = ((row as isize + delta_row) as u8) as char;
    col = ((col as isize + delta_col) as u8) as char;

    // If the new position is out of bounds, return false
    if !position_in_bounds(board, row, col) {
        return false;
    }

    // Convert the row and col to indices
    let row_idx = row as usize - 'a' as usize;
    let col_idx = col as usize - 'a' as usize;

    // Check if the new position is unoccupied or occupied by the same color
    if board[row_idx][col_idx] == colour || board[row_idx][col_idx] == '.' {
        return false;
    }

    // Continue moving in the specified direction
    row = ((row as isize + delta_row) as u8) as char;
    col = ((col as isize + delta_col) as u8) as char;

    // Keep checking until we go out of bounds
    while position_in_bounds(board, row, col) {
        let row_idx = row as usize - 'a' as usize;
        let col_idx = col as usize - 'a' as usize;

        // If we encounter an unoccupied space, the move is illegal
        if board[row_idx][col_idx] == '.' {
            return false;
        }
        // If we encounter the same color, the move is legal
        else if board[row_idx][col_idx] == colour {
            return true;
        }

        // Move to the next coordinate in the direction
        row = ((row as isize + delta_row) as u8) as char;
        col = ((col as isize + delta_col) as u8) as char;
    }

    // If we go out of bounds, the move is illegal
    false
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

fn count_final_score(board: &mut [[char; 8]; 8], colour: char) -> usize {
    let mut score = 0;

    // Loop through the board
    for i in 0..8 {
        for j in 0..8 {
            // Count occurrences of the given color
            if board[i][j] == colour {
                score += 1;
            }
        }
    }
    score
}

fn main() {
    //println!("Hello, world!");
    let mut board: [[char; 8]; 8] = [['.'; 8]; 8]; 
    let mut row = ' ';
    let mut col = ' ';
    print_board(&mut board);
}

