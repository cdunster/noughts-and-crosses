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
fn main() {
    println!("Welcome to Noughts and Crosses!");
    let side = select_side();
    draw_grid();
}
