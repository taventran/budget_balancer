mod keyword_structs;
use keyword_structs::Cli;

fn main() {
    // Getting arguments for command
    let category = std::env::args().nth(1).unwrap();
    let amount = std::env::args().nth(2).unwrap().parse::<i64>().unwrap();
    let mut num: i64 = 0;

    let test = Cli {
        category,
        amount,
    };

    println!("category: {:?}, amount: {:?}", test.category, test.amount);


}
