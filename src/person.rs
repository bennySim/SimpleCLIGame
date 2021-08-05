use rand::Rng;
use crate::valet::Valet;
use crate::generator::Generator;
use crate::commands::CommandFail;

pub(crate) struct Person {
    person_type: PersonType,
    pub(crate) name: String,
    ip_address: String,
    valet: Option<Valet>,
    defense_values: Vec<u8>,
    defense_no: u8,
    pub(crate) current_success: u8,
}

impl Person {
    pub(crate) fn generate() -> Option<Person> {
        let mut rng = rand::thread_rng();
        let prop = rng.gen_range(0..=10);
        let person_type = match prop {
            1..=6 => Some(PersonType::Common),
            7..=9 => Some(PersonType::Rare),
            10 => Some(PersonType::Epic),
            _ => None,
        };

        let generator = Generator::new();
        if let Some(person_type) = person_type {
            Some(Person {
                person_type,
                name: generator.get_random_name(),
                ip_address: generator.get_random_ip_address(),
                valet: Valet::generate(&person_type),
                defense_no: 0,
                defense_values: Person::get_defense_values(&person_type),
                current_success: 0,
            })
        } else {
            None
        }
    }
    pub fn get_defense(&mut self) -> u8 {
        let def = self.defense_values[self.defense_no as usize];
        self.defense_no = (self.defense_no + 1) % (self.defense_values.len() as u8);
        def
    }
    fn get_defense_values(person_type: &PersonType) -> Vec<u8> {
        match person_type {
            PersonType::Common => vec![0, 10],
            PersonType::Rare => vec![15],
            PersonType::Epic => vec![10, 15, 20]
        }
    }

    pub(crate) fn hack(&mut self, hacking_skill : u8) -> bool {
        let mut rng = rand::thread_rng();
        self.current_success += rng.gen_range(0..=hacking_skill);
        let def = self.get_defense();
        if (self.current_success as i16) - (def as i16) <= 0 {
            false
        } else {
            self.current_success -= def;
            true
        }
    }

    pub(crate) fn send(&mut self) -> Result<f64, CommandFail> {
        let mut rng = rand::thread_rng();
        let success = rng.gen_range(0..=100);
        if success > self.current_success {
            return Err(CommandFail::COMMAND_FAIL)
        }
        if let Some(valet) = &self.valet {
            Ok(valet.num_of_btc)
        } else {
            Err(CommandFail::NO_VALET)
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub(crate) enum PersonType {
    Common,
    Rare,
    Epic,
}