use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut calories: Vec<i32> = Vec::new();

    fs::read_to_string(file_path).expect("Couldn't open file")
        .split("\n")
        .fold(0, |sum, item| {
            match item.parse::<i32>() {
                Ok(calory) =>  sum+calory,
                Err(_err) => {calories.push(sum); 0}
            }
        }
    );
    
    let max_calories = calories.iter().max().unwrap();
    println!("{max_calories}");

    // Second part
    calories.sort();
    let top_three_sum: i32 = calories[calories.len()-3..].iter().sum();
    println!("{top_three_sum}")
}
