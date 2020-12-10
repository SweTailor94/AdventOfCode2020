use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let data = get_decl_from_file("src/day6_input.txt");
    // let mut count = 0;
    println!("Data length {}" , data.len());
    let val = data.iter().fold(0, | acc, set| acc + set.len() as i32 );
    println!("Sum {}",val);
}

pub fn get_decl_from_file(file_name:&str) -> Vec<HashSet<char>>{
    let mut declaration: Vec<HashSet<char>> = Vec::new();
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut current: HashSet<char> = HashSet::new();
    let mut new_group = true;
    loop {
        let mut line = String::new();
        let length = reader.read_line(&mut line).unwrap();
        if length == 0 {
            declaration.push(current);
            break;
        }
        let trimmed_line = line.trim();
        if trimmed_line.len() == 0 {
            // next passport
            declaration.push(current);
            current = HashSet::new();
            new_group = true;
        }else {
            // Part 1
            // add_cha
            // add_chars_to_set(trimmed_line, &mut current);

            // Part 2
            if new_group {
                add_chars_to_set(trimmed_line, &mut current);
                new_group = false;
            }
            else {

                current = add_intersection_of_chars_to_set(trimmed_line, &mut current );
            }
        }
    }
    declaration
}

fn add_chars_to_set(line: &str, current_set: &mut HashSet<char>)  {
        for c in line.chars()
        {
            current_set.insert(c);
        }
}

fn add_intersection_of_chars_to_set(line: &str, current_set: &mut HashSet<char>)  -> HashSet<char> {
    let mut temp_set: HashSet<char> = HashSet::new();
    for c in line.chars()
    {
        temp_set.insert(c);
    }
    let mut result = HashSet::new();
    for ch in current_set.intersection(&temp_set) {
        result.insert(*ch);
    }
    result
}