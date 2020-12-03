mod expense_report;
use crate::expense_report::EXPENSES;

fn get_product() -> u64{
    let count = EXPENSES.len();
    for i in 0..count {
        for k in (i + 1)..count {
            if EXPENSES[i] + EXPENSES[k] == 2020 {
                return EXPENSES[i] * EXPENSES[k];
            }
        }
    };
    0
}

fn get_product_part2() -> u64{
    let count = EXPENSES.len();
    for i in 0..count {
        for k in (i + 1)..count {
            for l in (k+1)..count{
                if EXPENSES[i] + EXPENSES[k]+ EXPENSES[l] == 2020 {
                    return EXPENSES[i] * EXPENSES[k] * EXPENSES[l];
                }
            }
        }
    };
    0
}

fn main() {


    println!("part1 {}",get_product());
    println!("part2 {}",get_product_part2());
}
