use rand::Rng;
use crate::person::PersonType;
use crate::generator::Generator;

#[derive(Debug)]
pub(crate) struct Valet {
    address: String,
    password: String,
    pub(crate) num_of_btc: f64,
}

impl Valet {
    pub(crate) fn new(num_of_btc: f64) -> Valet {
        let generator = Generator::new();
        Valet {
            address: generator.get_random_btc_address(),
            password: generator.get_random_password(),
            num_of_btc,
        }
    }

    pub(crate) fn generate(person_type: &PersonType) -> Option<Valet> {
        let mut rng = rand::thread_rng();
        if *person_type == PersonType::Common {
            let prop_of_valet = rng.gen_range(0.0..=1.0);
            if prop_of_valet < 0.25 {
                return None;
            }
        }

        let num_of_btc = match person_type {
            PersonType::Common => rng.gen_range(0.0..=0.5),
            PersonType::Rare => rng.gen_range(0.5..=1.5),
            PersonType::Epic => rng.gen_range(1.0..=2.5)
        };
        let generator = Generator::new();
        Some(Valet {
            address: generator.get_random_btc_address(),
            password: generator.get_random_password(),
            num_of_btc,
        })
    }

    pub fn subtract_btc(&mut self, num_of_btc: f64) -> bool {
        if num_of_btc > self.num_of_btc {
            return false;
        }
        self.num_of_btc -= num_of_btc;
        true
    }
}
