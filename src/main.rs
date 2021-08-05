pub mod person;
pub mod valet;
pub mod player;
pub mod generator;
pub mod commands;

use crate::player::Player;
use std::io;
use std::io::Read;
use crate::commands::{Command, CommandFail};
use std::str::FromStr;
use crate::person::Person;

fn main() {
    let mut stdin = io::stdin();
    let mut player = Player::new(String::from("Simona"));
    let mut people_to_hack: Vec<Person> = Vec::new();

    let mut line = String::new();
    loop {
        print!("> ");
        stdin.read_line(&mut line).unwrap();
        let command = Command::from_str(line.trim()).unwrap();
        match command {
            Command::FIND => find(&mut player, &mut people_to_hack),
            Command::HACK => hack(&mut player, &mut people_to_hack),
            Command::BRIBE => bribe(&mut player),
            Command::INFO => println!("{}", player.info()),
            Command::SEND => send(&mut player, &mut people_to_hack),
            Command::LEARN => learn(&mut player),
            _ => (),
        }
        line.clear();
        if command == Command::SURRENDER {
            return;
        }
    }
}

fn learn(player: &mut Player) {
    if player.learn() {
        println!("Your hacking skill increased to {}.", player.hacking_skill);
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
                Err(CommandFail::COMMAND_FAIL) => println!("You were discovered! Your criminality level increased to {}.", player.criminality_level),
                Err(CommandFail::NO_VALET) => println!("This person does not have valet."),
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
            println!("You are hacking {}", person.name);
            let res = player.hack(&mut person);
            if res {
                println!("Your hack was successful. Current success is {}.", person.current_success);
                people_to_hack.push(person);
            } else {
                println!("Your were discovered! Your criminality level increase to {}.", player.criminality_level);
            }
        }
    }
}

fn bribe(player: &mut Player) {
    if player.criminality_level == 0 {
        println!("Your criminality level is already 0!");
        return;
    }
    if player.bribe() {
        println!("Your criminality level decrease to {}", player.criminality_level);
    } else {
        println!("You don't have enough BTC to bribe!");
    }
}

fn find(player: &mut Player, people_to_hack: &mut Vec<Person>) {
    match player.find() {
        Ok(person) => {
            println!("You found {}!", person.name);
            people_to_hack.push(person);
        }
        Err(CommandFail::NO_PERSON) => println!("No person found!"),
        Err(CommandFail::NOT_ENOUGH_BTC) => println!("Not enough BTC to find someone!"),
        _ => println!("Not happening"),
    }
}
