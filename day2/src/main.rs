use crate::password_list::{get_passwords_w_rules, PasswordRuleAndValue};

mod password_list;

fn main() {
    let mut valid_passwords= get_passwords_w_rules("password.txt").into_iter().
        filter(|x|x.is_valid()) ;


    println!("There are {} valid passwords",valid_passwords.count());
}
