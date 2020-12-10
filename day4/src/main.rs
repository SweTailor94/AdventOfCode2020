use crate::passport::{get_passports_from_file, Passport};

mod passport;

fn main() {

    let pass = get_passports_from_file("src/day4_passport_data.txt");
    let valid_passports = pass.iter().filter(|p|p.is_valid()).count();
    println!("Part1 Number of valid passports {}",valid_passports);


  //  let pass = get_passports_from_file("src/invalid_passport_data.txt");
  //  let valid_passports_2 = pass.iter().filter(|p|p.is_valid2()).count();
  //  println!("Part2 Number of valid passports (expct 0){}",valid_passports_2);

    let pass = get_passports_from_file("src/day4_passport_data.txt");
    let valid_passports_2 = pass.iter().filter(|p|p.is_valid2()).count();
    println!("Part2 Number of valid passports {}",valid_passports_2);
}
