use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Slope{
    hill_grid: Vec<Vec<bool>>,
}

impl Slope {
    pub fn get_total_rows_in_slope(&self) -> usize{
        self.hill_grid.len()
    }

    pub fn get_pattern_width(&self) -> usize{
        self.hill_grid[0].len()
    }

    pub fn is_tree_at(&self, row:usize, col:usize) -> bool{
        let col_to_use = col % self.get_pattern_width();
        self.hill_grid[row][col_to_use]
    }
}

pub fn get_slope_from_input(file_name:&str) -> Slope {
    let mut the_hill : Vec<Vec<bool>> = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        the_hill.push(get_bools_from_line( line.unwrap().as_str()) );
    }
    Slope{
        hill_grid : the_hill
    }
}

fn get_bools_from_line(line:&str) -> Vec<bool>{
    let byte_line = line.as_bytes();
    let mut bools: Vec<bool> = Vec::new();
    for pos in byte_line.iter(){
        if *pos == '#' as u8 {
            bools.push(true); // A tree
        }
        else {
            bools.push(false) // open (no tree)
        }
    }
    bools
}