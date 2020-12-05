use crate::input_map::{get_slope_from_input, Slope};

mod input_map;

fn main() {

    let the_slope = get_slope_from_input("src/day3_input.txt");
    println!("Part 1: Number of trees {}", number_of_encountered_trees(3,1,&the_slope));

    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.
    let one = number_of_encountered_trees(1,1,&the_slope);
    let two = number_of_encountered_trees(3,1,&the_slope);
    let three = number_of_encountered_trees(5,1,&the_slope);
    let four = number_of_encountered_trees(7,1,&the_slope);
    let five = number_of_encountered_trees(1,2,&the_slope);
    print!("Part 2 r1d1 {}, r3d1 {}, r5d1 {}, r7d1 {}, r1d2 {}, product {}",
    one, two,three,four,five,one*two*three*four*five);
}

fn number_of_encountered_trees(right:usize, down:usize, the_slope: &Slope) -> u64{
    let slope_lengt = the_slope.get_total_rows_in_slope();
    let mut row:usize = 0;
    let mut col:usize = 0;
    let mut count=0;
    while row < slope_lengt {
        if the_slope.is_tree_at(row, col) {
            count += 1;
        }
        row += down;
        col += right;
    }
    count
}