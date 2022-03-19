use std::io;

mod number_game;
mod rps_game;

fn main() {
    println!("Which Game?");
    println!("1: Number Game?");
    println!("[2]: Rock Paper Scissos Game?");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 2
    };

    match choice {
        1 => number_game::play(),
        2 | _ => rps_game::play(),
    }
}
