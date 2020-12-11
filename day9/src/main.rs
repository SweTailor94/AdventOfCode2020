use crate::xmas_crypto::{get_stream_data, find_first_invalid_number, find_weakness};

mod xmas_crypto;

fn main() {
    let x = get_stream_data("src/day9_input.txt");
    match find_first_invalid_number(&x,25){
        Some(invalid) =>{
            println!("Part one. First invalid number {}",invalid);
            let weakness = find_weakness(&x,invalid);
            println!("Part two: {}", weakness);
        },
        None => println!("No invalid number found")
    }

}
