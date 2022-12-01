use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut calories: Vec<i32> = Vec::new();

    fs::read_to_string(file_path)
        .expect("Couldn't open file")
        .split("\n")
        .fold(0, |sum, item| match item.parse::<i32>() {
            Ok(calorie) => sum + calorie,
            Err(_err) => {
                calories.push(sum);
                0
            }
        });

    let max_calories = calories.iter().max().unwrap();
    println!("{max_calories}");

    // Second part
    let mut top_three: [i32; 3] = [0; 3];
    for calorie in calories {
        let smallest = top_three.iter().min().unwrap();
        let smallest_index = top_three.iter().position(|x| x == smallest).unwrap();
        if &calorie > smallest {
            top_three[smallest_index] = calorie
        }
    }
    let top_sum = top_three.iter().sum::<i32>();

    println!("{top_sum}")
}
