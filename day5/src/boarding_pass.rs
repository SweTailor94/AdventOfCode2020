use std::fs::File;
use std::io::{BufReader, BufRead};
#[derive(Debug, Clone)]
pub struct BoardingPass
{
    pub code:String,
    pub row:u8,
    pub col:u8,
}

impl BoardingPass {
    pub fn new(seat_code:String) -> BoardingPass{
        let row_code:&str = &seat_code[..7];
        let col_code:&str = &seat_code[7..];
        let row:u8 = get_row(row_code);
        let col:u8 = get_col(col_code);
        BoardingPass{
            code:row_code.to_string(),
            row,
            col
        }
    }

    pub fn get_seat_id(&self) -> u16{
        self.row as u16 * 8 + self.col as u16
    }
}

fn get_col(code: &str) -> u8 {
    let mut min:u8 = 0;
    let mut max : u8 = 7;
    for letter in code.chars(){
        match letter {
            'R' =>{ min =min + (max-min+1)/2;},
            'L' => {max =max - (max-min+1)/2;},
            _ => panic!("Invalid input in column specifier.")
        }
    }
    min
}

fn get_row(code:&str) -> u8 {
    let mut min:u8 = 0;
    let mut max : u8 = 127;
    for letter in code.chars(){
        match letter {
            'B' =>{ min =min + (max-min+1)/2;},
            'F' => {max =max - (max-min+1)/2;},
            _ => panic!("Invalid input in column specifier.")
        }
    }
    min

}


pub fn get_boarding_pass_from_file(file_name:&str) -> Vec<BoardingPass>{
    let mut boardingpasses: Vec<BoardingPass> = Vec::new();
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

        for line in  reader.lines()
        {
            boardingpasses.push(BoardingPass::new(line.unwrap()));
        };

    boardingpasses
}


#[cfg(test)]
mod tests {
    use crate::boarding_pass::{BoardingPass, get_col, get_row};

    #[test]
    fn test_get_col()
    {
        let col = get_col("RLR");
        assert_eq!(5,col);
        let col = get_col("RRR");
        assert_eq!(7,col);
        let col = get_col("RLL");
        assert_eq!(4,col);
        let col = get_col("LLL");
        assert_eq!(0,col);
    }

    #[test]
    fn test_get_row()
    {
        let row = get_row("FBFBBFF");
        assert_eq!(44,row);
        let row = get_row("BFFFBBF");
        assert_eq!(70,row);
        let row = get_row("FFFBBBF");
        assert_eq!(14,row);
        let row = get_row("BBFFBBF");
        assert_eq!(102,row);
        let row = get_row("FFFFFFF");
        assert_eq!(0,row);
        let row = get_row("BBBBBBB");
        assert_eq!(127,row);
    }

    #[test]
    fn test_boarding_pass_new(){
//        BFFFBBFRRR: row 70, column 7, seat ID 567.
        let p1 = BoardingPass::new("BFFFBBFRRR".to_string());
        assert_eq!(70,p1.row);
        assert_eq!(7,p1.col);
        assert_eq!(567,p1.get_seat_id());

//        FFFBBBFRRR: row 14, column 7, seat ID 119.
        let p1 = BoardingPass::new("FFFBBBFRRR".to_string());
        assert_eq!(14,p1.row);
        assert_eq!(7,p1.col);
        assert_eq!(119,p1.get_seat_id());
//        BBFFBBFRLL: row 102, column 4, seat ID 820.
        let p1 = BoardingPass::new("BBFFBBFRLL".to_string());
        assert_eq!(102,p1.row);
        assert_eq!(4,p1.col);
        assert_eq!(820,p1.get_seat_id());


    }
}