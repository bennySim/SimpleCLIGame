use crate::valet::Valet;
use crate::person::Person;
use crate::generator::Generator;
use crate::commands::CommandError;
use crate::commands::CommandError::{Discovered, NotEnoughBtc, NoPerson};

const BRIBE_PRICE: f64 = 0.05;
const LEARN_PRICE: f64 = 0.005;
pub(crate) const FIND_PRICE: f64 = 0.01;

#[derive(Debug)]
pub(crate) struct Player {
    name: String,
    ip_address: String,
    valet: Valet,
    hacking_skill: u8,
    criminality_level: u8,
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

    pub(crate) fn find(&mut self) -> Result<Person, CommandError> {
        if !self.subtract_btc(FIND_PRICE) {
            return Err(NotEnoughBtc);
        }
        let person = Person::generate();
        if let Some(person) = person {
            Ok(person)
        } else {
            Err(NoPerson)
        }
    }

    pub(crate) fn hack(&mut self, person: &mut Person) -> Result<u8, ()> {
        let hacking_result = person.hack(self.hacking_skill);
        if hacking_result.is_err() {
            self.criminality_level += 1;
        }

        hacking_result
    }

    pub(crate) fn send(&mut self, person: &mut Person) -> Result<f64, CommandError> {
        let result = person.send();

        match result {
            Ok(value) => self.add_btc(value), 
            Err(Discovered) => self.criminality_level += 1,
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

    pub(crate) fn learn(&mut self) -> Result<u8, ()> {
        if self.subtract_btc(LEARN_PRICE) {
            self.hacking_skill += 1;
            Ok(self.hacking_skill)
        } else {
            Err(())
        }
    }

    pub(crate) fn info(&self) -> String {
        format!("{:?}", self)
    }

    pub(crate) fn win(&self) -> bool {
        self.num_of_btc() >= 5.0
    }

    pub(crate) fn criminality_level(&self) -> u8 {self.criminality_level}

    pub(crate) fn num_of_btc(&self) -> f64 {self.valet.num_of_btc()}

    fn subtract_btc(&mut self, value_to_subtract: f64) -> bool {
        self.valet.subtract_btc(value_to_subtract)
    }
    fn add_btc(&mut self, value: f64) {
        self.valet.add_btc(value);
    }
}