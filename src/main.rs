extern crate rand;

use rand::Rng;
use std::io;

static mut GRID: [[char; 3]; 3] = [[' '; 3]; 3];

fn select_side() -> char {
    loop {
        println!("Would you like to be O's or X's?");
        let mut side = String::new();
        io::stdin().read_line(&mut side).unwrap();
        let side = side.trim().to_ascii_uppercase();
        match side.as_str() {
            "X" => return 'X',
            "O" => return 'O',
            _ => println!("{} is and invalid choice, please type O or X.", side),
        }
    }
}

fn draw_grid() {
    unsafe {
        println!("{}|{}|{}", GRID[0][0], GRID[0][1], GRID[0][2]);
        println!("_____");
        println!("{}|{}|{}", GRID[1][0], GRID[1][1], GRID[1][2]);
        println!("_____");
        println!("{}|{}|{}", GRID[2][0], GRID[2][1], GRID[2][2]);
    }
}

fn take_turn(side: char) {
    println!("Where would you like to place your {}?", side);
    loop {
        println!("Please enter as row [space] column.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut pos = input.split_ascii_whitespace();
        let row = pos.next().unwrap().parse::<usize>().unwrap();
        let col = pos.next().unwrap().parse::<usize>().unwrap();
        unsafe {
            if GRID[row][col] == ' ' {
                GRID[row][col] = side;
                break;
            } else {
                println!("Position is not valid.");
            }
        }
    }
}

fn computer_turn(side: char) {
    println!("Computer's turn.");
    loop {
        let row = rand::thread_rng().gen_range(0, 3);
        let col = rand::thread_rng().gen_range(0, 3);
        unsafe {
            if GRID[row][col] == ' ' {
                GRID[row][col] = side;
                break;
            }
        }
    }
}

fn is_row_won(row: usize) -> Option<char> {
    unsafe {
        if GRID[row][0] == GRID[row][1] && GRID[row][1] == GRID[row][2] && GRID[row][0] != ' ' {
            return Some(GRID[row][0]);
        }
    }

    None
}

fn is_column_won(col: usize) -> Option<char> {
    unsafe {
        if GRID[0][col] == GRID[1][col] && GRID[1][col] == GRID[2][col] && GRID[0][col] != ' ' {
            return Some(GRID[0][col]);
        }
    }
    None
}

fn is_diagonals_won() -> Option<char> {
    unsafe {
        if GRID[0][0] == GRID[1][1] && GRID[1][1] == GRID[2][2] && GRID[0][0] != ' ' {
            return Some(GRID[0][0]);
        } else if GRID[0][2] == GRID[1][1] && GRID[1][1] == GRID[2][0] && GRID[0][2] != ' ' {
            return Some(GRID[0][2]);
        }
    }
    None
}

fn find_winner() -> Option<char> {
    for i in 0..3 {
        if let Some(winner) = is_row_won(i) {
            return Some(winner);
        }

        if let Some(winner) = is_column_won(i) {
            return Some(winner);
        }

        if let Some(winner) = is_diagonals_won() {
            return Some(winner);
        }
    }

    None
}

fn main() {
    println!("Welcome to Noughts and Crosses!");
    let user_side = select_side();
    let mut computer_side = 'X';
    if let 'X' = user_side {
        computer_side = 'O';
    }
    loop {
        draw_grid();
        take_turn(user_side);
        computer_turn(computer_side);
        if let Some(winner) = find_winner() {
            if winner == user_side {
                println!("You won!");
            } else {
                println!("You lost!");
            }
            draw_grid();
            break;
        }
    }
}
