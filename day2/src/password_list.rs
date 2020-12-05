use std::fs::File;
use std::io::{BufReader, BufRead};


/// Parse the input file into a nice struct

pub struct PasswordRuleAndValue
{
    min:i32,
    max:i32,
    letter: String,
    password: String
}

impl PasswordRuleAndValue{

    pub fn is_valid(&self) -> bool{
        let chars:Vec<&str> = self.password.matches(&*self.letter).collect();
        return chars.len() as i32 >= self.min && chars.len() as i32 <= self.max ;
    }
    
    pub fn is_valid_part2(&self) -> bool{
        let pwd = self.password.as_bytes();
        let char = self.letter.as_bytes();

        pwd[self.min as usize - 1] != pwd[self.max as usize -1] &&
            (pwd[self.min as usize - 1] == char[0] || pwd[self.max as usize -1] == char[0] )
    }

    pub fn new( value:&str) -> PasswordRuleAndValue{
        let rule_pwd : Vec<&str> = value.split(":").collect();
        let limits_letter : Vec<&str> = rule_pwd[0].split(" ").collect();
        let min_max : Vec<&str>= limits_letter[0].split("-").collect();

        PasswordRuleAndValue {
            min : min_max[0].parse::<i32>().unwrap(),
            max : min_max[1].parse::<i32>().unwrap(),
            letter : limits_letter[1].trim().to_string(),
            password : rule_pwd[1].trim().to_string()
        }
    }
}

pub fn get_passwords_w_rules(file_name:&str) -> Vec<PasswordRuleAndValue> {
    let mut all_of_them : Vec<PasswordRuleAndValue> = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let val = PasswordRuleAndValue::new(line.unwrap().as_str());
        all_of_them.push(val);
    }
    return all_of_them;
}