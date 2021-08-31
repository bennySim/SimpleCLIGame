pub mod person;
pub mod valet;
pub mod player;
pub mod generator;
pub mod commands;

use crate::player::FIND_PRICE;
use crate::player::Player;
use std::io;
use std::io::{Write, Stdin};
use crate::commands::{Command, CommandError};
use std::str::FromStr;
use crate::person::Person;
use strum::IntoEnumIterator;

fn main() {
    let mut stdin = io::stdin();
    let mut player = create_player(&mut stdin);
    let mut people_to_hack: Vec<Person> = Vec::new();
    let mut line = String::new();
    let mut is_end = false;

    print_commands();

    while !is_end {
        is_end |= read_command(&mut stdin, &mut player, &mut people_to_hack, &mut line);
        is_end |= check_game_over(&mut player, &mut people_to_hack)
    }
}

fn create_player(stdin: &mut Stdin) -> Player {
    print!("Type your name: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    Player::new(String::from(line.trim()))
}

fn check_game_over(player: &mut Player, people_to_hack: &mut Vec<Person>) -> bool {
    if player.criminality_level() >= 5 {
        println!("Game over. Your criminality level is {}. ", player.criminality_level());
        return true;
    }

    if player.num_of_btc() < FIND_PRICE && people_to_hack.is_empty() {
        println!("Game over. Not enough BTC to find someone.");
        return true;
    }
    false
}

fn read_command(stdin: &mut Stdin, mut player: &mut Player, mut people_to_hack: &mut Vec<Person>, mut line: &mut String) -> bool {
    let mut is_end = false;
    print!("> ");
    stdin.read_line(&mut line).unwrap();
    let command = Command::from_str(line.trim());
    if let Ok(command) = command {
        is_end = execute_command(&mut player, &mut people_to_hack, command)
    } else {
        print!("Command '{}' is not supported. ", line.trim());
        print_commands();
    }
    line.clear();
    is_end
}

fn execute_command(mut player: &mut &mut Player, mut people_to_hack: &mut &mut Vec<Person>, command: Command) -> bool {
    let mut is_end = false;
    match command {
        Command::FIND => find(&mut player, &mut people_to_hack),
        Command::HACK => hack(&mut player, &mut people_to_hack),
        Command::BRIBE => bribe(&mut player),
        Command::INFO => println!("{}", player.info()),
        Command::SEND => send(&mut player, &mut people_to_hack),
        Command::LEARN => learn(&mut player),
        Command::WIN => win(&player),
        Command::SURRENDER => is_end = true,
        Command::COMMANDS => print_commands(),
    }
    is_end
}

fn print_commands() {
    print!("Supported commands are ");
    let mut first = true;
    for command in Command::iter() {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        let command = format!("{:?}", command).to_ascii_lowercase();
        print!("{}", command);
    }
    println!(".");
    io::stdout().flush().unwrap();
}

fn win(player: &Player) {
    if player.win() {
        println!("Congrats, you won!");
    } else {
        println!("You don't have enough BTC to win! At least 5 BTC is needed.");
    }
}

fn learn(player: &mut Player) {
    if let Ok(hacking_skill) = player.learn() {
        println!("Your hacking skill increased to {}.", hacking_skill);
    } else {
        println!("You don't have enough BTC for this command.");
    }
}

fn send(player: &mut Player, people_to_hack: &mut Vec<Person>) {
    let person = people_to_hack.pop();
    match person {
        None => println!("Nobody to send BTC from. Find someone first."),
        Some(mut person) => {
            let res = player.send(&mut person);
            match res {
                Ok(value) => println!("You successfully send {} BTC to your valet.", value),
                Err(CommandError::Discovered) => println!("You were discovered! Your criminality level increased to {}.", player.criminality_level()),
                Err(CommandError::NoValet) => println!("This person does not have valet."),
                _ => (),
            }
        }
    }
}

fn hack(player: &mut Player, people_to_hack: &mut Vec<Person>) {
    let person: Option<Person> = people_to_hack.pop();
    match person {
        None => println!("Nobody to hack. Find someone first."),
        Some(mut person) => {
            println!("You are hacking {}", person.name());
            let hacking_result = player.hack(&mut person);
            if let Ok(current_success) = hacking_result {
                println!("Your hack was successful. Current success is {}.", current_success);
                people_to_hack.push(person);
            } else {
                println!("Your were discovered! Your criminality level increase to {}.", player.criminality_level());
            }
        }
    }
}

fn bribe(player: &mut Player) {
    if player.criminality_level() == 0 {
        println!("Your criminality level is already 0!");
        return;
    }
    if player.bribe() {
        println!("Your criminality level decrease to {}", player.criminality_level());
    } else {
        println!("You don't have enough BTC to bribe!");
    }
}

fn find(player: &mut Player, people_to_hack: &mut Vec<Person>) {
    match player.find() {
        Ok(person) => {
            println!("You found {}!", person.name());
            people_to_hack.push(person);
        }
        Err(CommandError::NoPerson) => println!("No person found!"),
        Err(CommandError::NotEnoughBtc) => println!("Not enough BTC to find someone!"),
        _ => println!("Not happening"),
    }
}
