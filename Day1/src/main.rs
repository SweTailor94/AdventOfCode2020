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

fn main() {


    println!("{}",get_product());
}
