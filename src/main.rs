use rand::Rng;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Hello, welcome to play my RPS in Python!\n\n");

    println!("What do ya say? :)");
    let choice_list: Vec<&str> = vec!["R", "P", "S"];
    let mut symbol_meanings: HashMap<&str, &str> = HashMap::new();
    symbol_meanings.insert("R", "Rock");
    symbol_meanings.insert("P", "Paper");
    symbol_meanings.insert("S", "Scissors");

    for (symbol, meaning) in &symbol_meanings {
        println!("{}: {}", symbol, meaning);
    }

    let run_again: bool = true;

    while run_again {
        let mut player_choice = String::new();

        let random_index = rand::thread_rng().gen_range(0..choice_list.len());

        let computer_choice = choice_list[random_index];

        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line");

        println!("You guessed: {}", player_choice.trim());
        println!("The computer guesses: {}", computer_choice);

        let player_choice_str = player_choice.trim();

        if choice_list.contains(&player_choice_str) {
            if player_choice_str == computer_choice {
                println!("Both players selected: {}\nIt is a tie!", player_choice);
                continue;
            } else if player_choice_str == "R" {
                if computer_choice == "S" {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                    println!("\nRock smashes Scissors ✂ ! You win!");
                } else {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                    println!("\nPaper covers Rock! CPU wins!");
                }
            } else if player_choice_str == "P" {
                if computer_choice == "R" {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                    println!("\nPaper covers Rock! You win!");
                } else {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                    println!("\nScissors cuts Paper! CPU wins!");
                }
            } else if player_choice_str == "S" {
                if computer_choice == "P" {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                } else {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                }
            } else if player_choice_str == "x" {
                if computer_choice == "x" {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                    println!("\nScissors cuts Paper! You win!");
                } else {
                    println!(
                        "Player selected: {}. \nComputer selected: {}.",
                        player_choice, computer_choice
                    );
                    println!("\nRock smashes Scissors! CPU wins!");
                }
            }

            break;
        } else {
            println!("Invalid input, please try again!");
            continue;
        }
    }
}
