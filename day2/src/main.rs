use crate::password_list::{get_passwords_w_rules};
use std::env::current_dir;

mod password_list;

fn main() {
    let passwords=
        get_passwords_w_rules("src/passwords.txt");

    let valid_passwords = passwords.into_iter().
        filter(|x|x.is_valid()) ;



    println!("There are {} valid passwords in part 1",valid_passwords.count());

    let passwords = get_passwords_w_rules("src/passwords.txt");
    let valid_passwords_part2  = passwords.into_iter().
        filter(|x|x.is_valid_part2()) ;
    println!("There are {} valid passwords in part 2",valid_passwords_part2.count());
}
