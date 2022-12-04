use ::std::env;
use ::std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let inputs_raw = fs::read_to_string(file_path).expect("Couldn't open the file");

    let input = inputs_raw
        .lines()
        .map(|line| {
            line.split(",")
                .map(|sections| sections.split("-").map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

    let overlaps: i32 = input
        .iter()
        .filter_map(|sections| {
            if ((sections[0][0] >= sections[1][0]) && (sections[0][1] <= sections[1][1]))
                || ((sections[0][0] <= sections[1][0]) && (sections[0][1] >= sections[1][1]))
            {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    println!("{overlaps}");

    // part two
    let real_overlaps: i32 = input
        .iter()
        .filter_map(|sections| {
            if ((sections[0][0] >= sections[1][0]) && (sections[0][0] <= sections[1][1])) || ((sections[0][1] >= sections[1][0]) && (sections[0][1] <= sections[1][1])) {
                Some(1)
            } else if ((sections[1][0] >= sections[0][0]) && (sections[1][0] <= sections[0][1])) || ((sections[1][1] >= sections[0][0]) && (sections[1][1] <= sections[0][1])) {
                Some(1)
            } else {
                None
            }
        }).sum();

    println!("{real_overlaps}");
}
