use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

#[derive( Debug,Clone)]
pub struct Passport{
    byr : Option<String>, //(Birth Year)
    iyr : Option<String>, //(Issue Year)
    eyr : Option<String>, //(Expiration Year)
    hgt : Option<String>, //(Height)
    hcl : Option<String>, //(Hair Color)
    ecl : Option<String>, //(Eye Color)
    pid : Option<String>, //(Passport ID)
    cid : Option<String>, //(Country ID)
}


impl Passport {

    pub fn is_valid(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some()  && self.eyr.is_some()
            && self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some()
            && self.pid.is_some() // cid is temporary not needed.
    }
    pub fn is_valid2(&self) -> bool {
        let valid = match &self.byr {// byr (Birth Year) - four digits; at least 1920 and at most 2002.
            Some(val) => Passport::evaluate_year(val,1920,2002),
            None => false
        } && match &self.iyr {            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            Some(val) => Passport::evaluate_year(val,2010,2020),
            None => false,
        } && match &self.eyr {            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            Some(value) => Passport::evaluate_year(value,2020,2030),
            None => false,
        } && match &self.hgt {            // hgt (Height) - a number followed by either cm or in:
            //     If cm, the number must be at least 150 and at most 193.
            //     If in, the number must be at least 59 and at most 76.
            Some(value) => {
                let len = value.len();
                let unit = &value[len-2..];
                let number = &value[..(len - 2)];
                let h:u8 = number.parse::<u8>().unwrap();
                match unit {
                    "cm" => 150 <= h && h <= 193,
                    "in" => 59 <= h && h <= 76,
                    _ => false,
                }
            },
            None => false,
        } && match &self.hcl {            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            Some(value) => {
                let re = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
                let ret = re.is_match(value);
                ret
            },
            None => false
        } && match &self.ecl {            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            Some(value) => {
                match value.as_str() {
                    "amb" |"blu" | "brn"|"gry"| "grn" |"hzl"| "oth" => true,
                    _ => false,
                }
            },
            None => false
        } && match &self.pid {            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            Some(value) => {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                let ret = re.is_match(value);
                ret
            },
            None => false
        }; // cid is temporary not needed.
        valid
    }

    fn evaluate_year(val: &String, min:u32, max:u32) -> bool {
        if val.len() != 4 { return false; }
        let year = val.parse::<u32>().unwrap();
        min <= year && year <= max
    }

    pub fn new() -> Passport{
        Passport{
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        }
    }

    pub fn set_value(&mut self, value_pair:&str){
        if  ! value_pair.contains(":") {panic!("Can't assign {}", value_pair);}
        let mut iter = value_pair.split(":");
        match iter.next() {
            Some("byr") => self.byr = Some(iter.next().unwrap().to_string()),
            Some("iyr") => self.iyr = Some(iter.next().unwrap().to_string()),
            Some("eyr") => self.eyr = Some(iter.next().unwrap().to_string()),
            Some("hgt") => self.hgt = Some(iter.next().unwrap().to_string()),
            Some("hcl") => self.hcl = Some(iter.next().unwrap().to_string()),
            Some("ecl") => self.ecl = Some(iter.next().unwrap().to_string()),
            Some("pid") => self.pid = Some(iter.next().unwrap().to_string()),
            Some("cid") => self.cid = Some(iter.next().unwrap().to_string()),
            Some(_) =>{},
            None => {}
        }
    }
}


pub fn get_passports_from_file(file_name:&str) -> Vec<Passport>{
    let mut passports: Vec<Passport> = Vec::new();
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut current_passport : Passport = Passport::new();
    loop {
        let mut line = String::new();
        let length = reader.read_line(&mut line).unwrap();
        if length == 0 {
            passports.push(current_passport);
            break;
        }
        if length == 1 {
            // next passport
            passports.push(current_passport);
            current_passport = Passport::new();
        }else {
            // Parse line into current_passport
            for pair in line.split(" "){
                current_passport.set_value(pair.trim());
            }
        }
    }
    passports
}