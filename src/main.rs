use std::io;

static GRID_SIZE: usize = 9;

fn print_board(array: [[usize; 9]; 9]) {
    for i in 0..array.len() {
        if i % 3 == 0 {
            println!("-------------");
        }
        for j in 0..array[i].len() {
            if j % 3 == 0 {
                print!("|");
            }
            print!("{}", array[i][j]);
        }
        print!("|");
        println!();
    }
    println!("-------------");
}

fn is_number_in_row(board: &mut [[usize; 9]; 9], number: usize, row: usize) -> bool {
    for i in 0..GRID_SIZE {
        if board[row][i] == number {
            return true
        }
    }
    false
}

fn is_number_in_column(board: &mut [[usize; 9]; 9], number: usize, column: usize) -> bool {
    for i in 0..GRID_SIZE {
        if board[i][column] == number {
            return true
        }
    }
    false
}

fn is_number_in_box(board: &mut [[usize; 9]; 9], number: usize, row: usize, column: usize) -> bool {
    let local_box_row = row - row % 3;
    let local_box_column = column - column % 3;

    for i in local_box_row..local_box_row + 3 {
        for j in local_box_column..local_box_column + 3 {
            if board[i][j] == number {
                return true;
            }
        }
    }
    false
}

fn is_valid_placement(board: &mut [[usize; 9]; 9], number: usize, row: usize, column: usize) -> bool {
    return !is_number_in_row(board, number, row) && 
        !is_number_in_column(board, number, column) &&
        !is_number_in_box(board, number, row, column);
}

fn solve_board(board: &mut [[usize; 9]; 9]) -> bool {
    for row in 0..GRID_SIZE {
        for column in 0..GRID_SIZE {
            if board[row][column] == 0 {
                for number_to_try in 1..=GRID_SIZE {
                    if is_valid_placement(board, number_to_try, row, column) {
                        board[row][column] = number_to_try;

                        if solve_board(board) {
                            return true;
                        } else {
                            board[row][column] = 0;
                        }
                    }
                }
                return false;
            }
        }
    }
    return true;
}

fn change_number(array: &mut [[usize; 9]; 9],column: usize, row: usize, number: usize) {
    array[row][column] = number;
}

fn main() {
    //let mut board: [[i32; 9]; 9] = [[0; 9]; 9]; // 9 x 9
    let mut board: [[usize; 9]; 9] = [
            [0, 0, 0, 0, 8, 0, 0, 0, 9],
            [0, 6, 0, 9, 0, 0, 1, 8, 0],
            [0, 0, 4, 0, 3, 0, 0, 0, 0],
            [1, 0, 0, 5, 0, 4, 0, 0, 6],
            [0, 0, 0, 3, 0, 7, 5, 4, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [5, 0, 7, 0, 0, 0, 0, 1, 0],
            [8, 4, 0, 0, 0, 3, 0, 0, 0],
            [0, 0, 9, 0, 0, 0, 7, 0, 2],
    ];

    let mut user_input = String::new();
    while user_input.trim() != "q" {
        //std::process::Command::new("clear").status().unwrap();
        print!("\x1B[2J\x1B[1;1H");
        print_board(board);

        user_input.clear();
        io::stdin().read_line(&mut user_input).unwrap();

        let args: Vec<&str> = user_input.trim().split(" ").collect();
        if args[0] == "change" {
            change_number(&mut board, args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap());
        }

        if args[0] == "solve" {
            if solve_board(&mut board) {
                println!("Solved");
            } else {
                println!("Failed");
            }
        }

    }
}
