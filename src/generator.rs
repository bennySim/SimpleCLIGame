use std::fs;
use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};

static FILE_NAME: &str = "/home/simona/git/CyberGameCLI/src/gameData.csv";

pub(crate) struct Generator {
    data: Vec<String>
}

impl Generator {
    pub fn new() -> Generator {
        let file = File::open(FILE_NAME).expect("Game data file does not exist.");
        let buf = BufReader::new(file);
        Generator {
            data: buf.lines().map(|l| l.expect("Could not parse line")).collect()
        }
    }

    pub fn get_random_name(&self) -> String {self.get_random_element().name}
    pub fn get_random_ip_address(&self) -> String {self.get_random_element().ip}
    pub fn get_random_btc_address(&self) -> String {self.get_random_element().btc_address}
    pub fn get_random_password(&self) -> String {self.get_random_element().password}

    fn get_random_element(&self) -> Line {
        let mut rng = rand::thread_rng();
        let line_no = rng.gen_range(1..self.data.len());
        let line : String = self.data[line_no].clone();
        let line : Vec<String> = line.split(',').map(|f| String::from(f.clone())).collect();
        Line {
            name : line[0].clone(),
            ip : line[1].clone(),
            btc_address: line[2].clone(),
            password: line[3].clone(),
        }

    }
}

struct Line {
    name: String,
    ip: String,
    btc_address: String,
    password: String,
}