use ::std::env;
use ::std::fs;
use std::collections::HashSet;

fn calculate_step(long_tail_coordinates: &mut [(i32, i32)], i: usize) {
    let head = long_tail_coordinates[i - 1];
    let mut tail = long_tail_coordinates[i];

    let x_dif = head.0 - tail.0;
    let y_dif = head.1 - tail.1;

    if x_dif.abs() > 1 {
        match y_dif {
            2       => tail.1 += 1,
            1 | -1  => tail.1 = head.1,     
            -2      => tail.1 -= 1,
            _       => (),
        }

        if x_dif > 0 { tail.0 += 1 } 
        else { tail.0 -= 1 }

    } else if y_dif.abs() > 1 {
        match x_dif {
            2       => tail.0 += 1,
            1 | -1  => tail.0 = head.0,
            -2      => tail.0 -= 1,
            _       => (),
        }

        if y_dif > 0 { tail.1 += 1 } 
        else { tail.1 -= 1 }
    }

    long_tail_coordinates[i] = (tail.0, tail.1)
}

fn step(
    long_tail_coordinates: &mut [(i32, i32)],
    (direction, number_of_steps): (&str, i32),
    visited_by_tail: &mut HashSet<(i32, i32)>,
) {
    for _ in 0..number_of_steps {
        match direction {
            "R" => {
                long_tail_coordinates[0].0 += 1;
                for i in 1..long_tail_coordinates.len() {
                    calculate_step(long_tail_coordinates, i);
                }
            }
            "L" => {
                long_tail_coordinates[0].0 -= 1;
                for i in 1..long_tail_coordinates.len() {
                    calculate_step(long_tail_coordinates, i);
                }
            }
            "U" => {
                long_tail_coordinates[0].1 += 1;
                for i in 1..long_tail_coordinates.len() {
                    calculate_step(long_tail_coordinates, i);
                }
            }
            "D" => {
                long_tail_coordinates[0].1 -= 1;
                for i in 1..long_tail_coordinates.len() {
                    calculate_step(long_tail_coordinates, i);
                }
            }
            _ => println!("The imput is corrupted"),
        }

        visited_by_tail.insert(*long_tail_coordinates.last().unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let raw_input = fs::read_to_string(file_path).expect("Couldn't open file");

    // part one
    let mut short_tail_coordinates = [(0, 0); 2];
    let mut visited_by_short_tail = HashSet::new();
    visited_by_short_tail.insert(short_tail_coordinates[1]);

    // part two
    let mut long_tail_coordinates = [(0, 0); 10];
    let mut visited_by_long_tail = HashSet::new();
    visited_by_long_tail.insert(long_tail_coordinates[9]);

    for line in raw_input.lines() {
        let motions: Vec<&str> = line.split(' ').collect();
        let (direction, number_of_steps) = (motions[0], motions[1].parse::<i32>().unwrap());

        // part one
        step(
            &mut short_tail_coordinates,
            (direction, number_of_steps),
            &mut visited_by_short_tail,
        );

        // part two
        step(
            &mut long_tail_coordinates,
            (direction, number_of_steps),
            &mut visited_by_long_tail,
        );
    }

    // part one
    let result = visited_by_short_tail.len();
    println!("{result}");

    // part two
    let result_two = visited_by_long_tail.len();
    println!("{result_two}");
}
