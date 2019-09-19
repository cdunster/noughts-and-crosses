extern crate rand;

use rand::Rng;
use std::io;

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

fn draw_grid(grid: &[[char; 3]; 3]) {
    println!("{}|{}|{}", grid[0][0], grid[0][1], grid[0][2]);
    println!("_____");
    println!("{}|{}|{}", grid[1][0], grid[1][1], grid[1][2]);
    println!("_____");
    println!("{}|{}|{}", grid[2][0], grid[2][1], grid[2][2]);
}

fn take_turn(side: char, grid: &mut [[char; 3]; 3]) {
    println!("Where would you like to place your {}?", side);
    loop {
        println!("Please enter as row [space] column.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut pos = input.split_ascii_whitespace();
        let row = pos.next().unwrap().parse::<usize>().unwrap();
        let col = pos.next().unwrap().parse::<usize>().unwrap();
        if grid[row][col] == ' ' {
            grid[row][col] = side;
            break;
        } else {
            println!("Position is not valid.");
        }
    }
}

fn computer_turn(side: char, grid: &mut [[char; 3]; 3]) {
    println!("Computer's turn.");
    loop {
        let row = rand::thread_rng().gen_range(0, 3);
        let col = rand::thread_rng().gen_range(0, 3);
        if grid[row][col] == ' ' {
            grid[row][col] = side;
            break;
        }
    }
}

fn is_row_won(row: usize, grid: &[[char; 3]; 3]) -> Option<char> {
    if grid[row][0] == grid[row][1] && grid[row][1] == grid[row][2] && grid[row][0] != ' ' {
        return Some(grid[row][0]);
    }
    None
}

fn is_column_won(col: usize, grid: &[[char; 3]; 3]) -> Option<char> {
    if grid[0][col] == grid[1][col] && grid[1][col] == grid[2][col] && grid[0][col] != ' ' {
        return Some(grid[0][col]);
    }
    None
}

fn is_diagonals_won(grid: &[[char; 3]; 3]) -> Option<char> {
    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[0][0] != ' ' {
        return Some(grid[0][0]);
    } else if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[0][2] != ' ' {
        return Some(grid[0][2]);
    }
    None
}

fn find_winner(grid: &[[char; 3]; 3]) -> Option<char> {
    for i in 0..3 {
        if let Some(winner) = is_row_won(i, grid) {
            return Some(winner);
        }

        if let Some(winner) = is_column_won(i, grid) {
            return Some(winner);
        }

        if let Some(winner) = is_diagonals_won(grid) {
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
    let mut grid: [[char; 3]; 3] = [[' '; 3]; 3];
    loop {
        draw_grid(&grid);
        take_turn(user_side, &mut grid);
        computer_turn(computer_side, &mut grid);
        if let Some(winner) = find_winner(&grid) {
            if winner == user_side {
                println!("You won!");
            } else {
                println!("You lost!");
            }
            draw_grid(&grid);
            break;
        }
    }
}
