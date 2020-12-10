use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use crate::hand_held_device::Instruction::{Nop, Jmp, Acc};

#[derive(Clone)]
pub enum Instruction{
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}
pub fn try_to_fix_program(file_name: &str) -> (i32,bool, usize) {
    let original_program = load_program_from_file(file_name);
    let mut instruction_pointer:usize = 0;
    loop {
        let mut test_program = original_program.clone();
        loop {
            match test_program[instruction_pointer] {
                Acc(_) => instruction_pointer += 1,
                Jmp(arg) => {
                    test_program[instruction_pointer] = Nop(arg);
                    break;
                },
                Nop(arg) => {
                    test_program[instruction_pointer] = Jmp(arg);
                    break;
                }
            }
        }
        match run_program_to_breakpoint_or_end(&test_program) {
            (acc, true) => return (acc, true, instruction_pointer),
            (_, false) => instruction_pointer += 1,
        }
        if instruction_pointer == test_program.len() {
            return (0, false, instruction_pointer);
        }
    }
}
pub fn load_program_from_file(file_name:&str) -> Vec<Instruction> {
    let mut program_memory: Vec<Instruction> = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_string = line.unwrap();
        let mut line_split = line_string.split(" ");
        match (line_split.next(), line_split.next()) {
            (Some("acc"),Some(arg)) => program_memory.push(Instruction::Acc(arg.parse().unwrap())),
            (Some("jmp"),Some(arg)) => program_memory.push(Instruction::Jmp(arg.parse().unwrap())),
            (Some("nop"),Some(arg)) => program_memory.push(Instruction::Nop(arg.parse().unwrap())),
            (_,_) => panic!("Unknown instruction"),
        }
    }
    return program_memory;
}

pub fn run_program_to_breakpoint_or_end(program:&Vec<Instruction>) -> (i32, bool){
    let mut program_counter:i32 = 0;
    let mut acc:i32 = 0;
    let mut visited_addresses: HashSet<usize> = HashSet::new();
    loop{
        if ! visited_addresses.insert(program_counter as usize){
            // Breakpoint, we been to this address before
            return (acc, false);
        }
        // Execute current program step
        match program[program_counter as usize] {
            Instruction::Acc(value) => {
                acc += value;
                program_counter+=1;
            }
            Instruction::Jmp(step) => {program_counter  += step;}
            Instruction::Nop(_) => {program_counter += 1;}
        }
        if program_counter as usize == program.len(){
            return (acc,true);
        }
    }
}