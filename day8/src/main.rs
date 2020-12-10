use crate::hand_held_device::{load_program_from_file, run_program_to_breakpoint_or_end,try_to_fix_program};

mod hand_held_device;

fn main() {
    let program = load_program_from_file("src/day8_input.txt");
    println!("------ Part one -------");
    match  run_program_to_breakpoint_or_end(&program){
        (acc, true) => println!("acc is {} at end of program", acc),
        (acc, false) => println!("acc is {} at breakpoint", acc)
    }
    println!("------ Part two -------");
    match try_to_fix_program("src/day8_input.txt"){
        (acc, true, address) => print!("Boot fixed acc is {}, (instruction @ {} changed)", acc, address),
        (_,false,_) => print!("Could not fix boot !"),
    }
}


