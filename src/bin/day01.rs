use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut sum = 0;
    let mut most_calories = 0;

    let input_raw = fs::read_to_string(file_path).expect("Couldn't open file");
    let input: Vec<&str> = input_raw.split("\n").collect();

    for line in &input {
        if *line != "" {sum += line.parse::<i32>().unwrap()}
        else if sum > most_calories {most_calories = sum; sum = 0}
        else { sum = 0 }
    }
    
    println!("{most_calories}");

    // part two
    sum = 0;
    let mut top_three = [0; 3];
    for line in &input {
        if *line != "" {sum += line.parse::<i32>().unwrap()}
        else if sum > *top_three.iter().min().unwrap() {
            let smallest_index = top_three.iter().position(|x| x == top_three.iter().min().unwrap()).unwrap();
            top_three[smallest_index] = sum;
            sum = 0
        }
        else { sum = 0 }
    }
    
    let top_three_sum: i32 = top_three.iter().sum();
    
    println!("{top_three_sum}")
}
