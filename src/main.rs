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
fn main() {
    println!("Welcome to Noughts and Crosses!");
    let side = select_side();
}
