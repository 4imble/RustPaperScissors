use num_enum::TryFromPrimitive;
use rand::Rng;
use std::io;

pub fn play() {
    println!("ROCK PAPER SCISSORS TIME!!");

    loop {
        let player_1_name = String::from("AI");
        let player_2_name = String::from("Human");

        println!("Make your move:");
        println!("(R)ock, (P)aper, (S)cissors");

        let secret_move = rand::thread_rng().gen_range(1..=3);
        let secret_move = Move::try_from(secret_move);
        let secret_move = secret_move.unwrap();

        //println!("{:?}", secret_move);

        let mut player_move = String::new();

        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        println!("------------------");

        let player_move = player_move.trim();

        let player_move = match player_move {
            "R" | "r" => Move::Rock,
            "P" | "p" => Move::Paper,
            "S" | "s" => Move::Scissors,
            _ => Move::None,
        };

        let game = Game {
            player1: Player {
                name: player_1_name,
                chosen_move: secret_move,
            },
            player2: Player {
                name: player_2_name,
                chosen_move: player_move,
            },
        };

        game.calculate_win();
        println!("------------------");
    }
}

struct Game {
    player1: Player,
    player2: Player,
}

impl Game {
    fn calculate_win(&self) {
        if self.player1.chosen_move == self.player2.chosen_move {
            let chosen_move = &self.player1.chosen_move;
            println!("Draw! ({:?})", chosen_move);
        } else if self.player2.chosen_move == Move::None {
            println!("Invalid Move!");
        } else {
            let winning_name: &str;

            match self.player1.chosen_move {
                Move::Rock => {
                    if self.player2.chosen_move == Move::Scissors {
                        winning_name = &self.player1.name
                    } else {
                        winning_name = &self.player2.name
                    }
                }
                Move::Paper => {
                    if self.player2.chosen_move == Move::Rock {
                        winning_name = &self.player1.name
                    } else {
                        winning_name = &self.player2.name
                    }
                }
                Move::Scissors => {
                    if self.player2.chosen_move == Move::Paper {
                        winning_name = &self.player1.name
                    } else {
                        winning_name = &self.player2.name
                    }
                }
                Move::None => winning_name = "Error",
            }

            println!(
                "{} Wins! ({:?} vs. {:?})",
                winning_name, self.player2.chosen_move, self.player1.chosen_move
            );
        };
    }
}

struct Player {
    name: String,
    chosen_move: Move,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Move {
    None,
    Rock,
    Paper,
    Scissors,
}