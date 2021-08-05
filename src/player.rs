use crate::valet::Valet;
use crate::person::Person;
use crate::generator::Generator;
use crate::commands::CommandFail;
use crate::commands::CommandFail::{COMMAND_FAIL, NOT_ENOUGH_BTC, NO_PERSON};

const BRIBE_PRICE: f64 = 0.05;
const LEARN_PRICE: f64 = 0.005;
const FIND_PRICE: f64 = 0.01;

#[derive(Debug)]
pub(crate) struct Player {
    name: String,
    ip_address: String,
    valet: Valet,
    pub(crate) hacking_skill: u8,
    pub(crate) criminality_level: u8, //TODO enum
}

impl Player {
    pub(crate) fn new(name: String) -> Player {
        Player {
            name,
            ip_address: Generator::new().get_random_ip_address(),
            valet: Valet::new(0.05),
            hacking_skill: 26,
            criminality_level: 0,
        }
    }

    pub(crate) fn find(&mut self) -> Result<Person, CommandFail> {
        if !self.subtract_btc(FIND_PRICE) {
            return Err(NOT_ENOUGH_BTC);
        }
        let person = Person::generate();
        if let Some(person) = person {
            Ok(person)
        } else {
            Err(NO_PERSON)
        }
    }

    pub(crate) fn hack(&mut self, person: &mut Person) -> bool {
        if person.hack(self.hacking_skill) {
            true
        } else {
            self.criminality_level += 1;
            false
        }
    }

    pub(crate) fn send(&mut self, person: &mut Person) -> Result<f64, CommandFail> {
        let result = person.send();

        match result {
            Ok(value) => self.valet.num_of_btc += value,
            Err(CommandFail::COMMAND_FAIL) => self.criminality_level += 1,
            _ => (),
        }
        result
    }

    pub(crate) fn bribe(&mut self) -> bool {
        if self.subtract_btc(BRIBE_PRICE) {
            self.criminality_level -= 1;
            return true;
        }
        false
    }

    pub(crate) fn learn(&mut self) -> bool {
        if self.subtract_btc(LEARN_PRICE) {
            self.hacking_skill += 1;
            return true;
        }
        false
    }

    pub(crate) fn info(&self) -> String {
        format!("{:?}", self)
    }

    fn win(&self) -> bool {
        if self.valet.num_of_btc >= 5.0 {
            true
        } else {
            false
        }
    }

    fn surrender(&self) -> bool {
        true
    }

    fn subtract_btc(&mut self, value_to_subtract: f64) -> bool {
        return self.valet.subtract_btc(value_to_subtract);
    }
}