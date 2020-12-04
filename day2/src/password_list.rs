

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

    pub fn new(&mut self, value:&str){
        let s : Vec<&str> = value.split(":").collect();
        let rule = s[0];
        self.password = s[1].trim().to_string();
    }
}

pub fn get_passwords_w_rules(file_name:&str) -> Vec<PasswordRuleAndValue> {
    let  all_of_them : Vec<PasswordRuleAndValue> = Vec::new();

    return all_of_them;
}