use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter)]
pub(crate) enum Command {
    FIND,
    HACK,
    SEND,
    BRIBE,
    LEARN,
    INFO,
    WIN,
    SURRENDER,
    COMMANDS,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
       match input.to_ascii_lowercase().as_str() {
           "find" => Ok(Command::FIND),
           "hack" => Ok(Command::HACK),
           "send" => Ok(Command::SEND),
           "bribe" => Ok(Command::BRIBE),
           "learn" => Ok(Command::LEARN),
           "info" => Ok(Command::INFO),
           "win" => Ok(Command::WIN),
           "surrender" => Ok(Command::SURRENDER),
           "commands" => Ok(Command::COMMANDS),
           _ => Err(()),
       }
    }
}

pub(crate) enum CommandError {
    Discovered,
    NoValet,
    NotEnoughBtc,
    NoPerson,
}