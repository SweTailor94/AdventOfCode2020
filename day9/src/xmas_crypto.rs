use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn find_weakness(data:&Vec<u128>, first_invalid:u128) -> u128{
    let mut first : usize = 0;
    let mut last : usize = 1;
    let mut sum :u128 = data[first]+data[last];
    loop{
        if sum == first_invalid && first != last {
            return sum_of_smallest_and_largest(&data[first..last+1]);
        }else if sum < first_invalid {
            last+=1;
            sum += data[last];
        }else {
            sum -= data[first];
            first+=1;
            if first > last {panic!("Can't find a weakness.");}
        }
    }
}

fn sum_of_smallest_and_largest(p0: &[u128]) -> u128 {
    let small = *p0.iter().min().unwrap();
    let large = *p0.iter().max().unwrap();
    small + large
}

pub fn find_first_invalid_number(data:&Vec<u128>, preamble:usize) -> Option<u128>{
    for i in 0..data.len()-1-preamble {
        let number_to_check = data[preamble+i];
        if ! find_two(&data[i..preamble+i],number_to_check ){
            return Some(number_to_check);
        }
    }
    None
}


fn find_two(window: &[u128], sum:u128) -> bool{
    let len = window.len();
    for i in 0 ..len-1{
        for j in i+1..len {
            if window[i]+window[j] == sum {
                return true;
            }
        }
    }
    false
}

pub fn get_stream_data(file_name:&str) -> Vec<u128> {
    let mut data_stream = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        data_stream.push(line.unwrap().parse().unwrap());
    }

    data_stream
}