use ::std::env;
use ::std::fs;
use std::collections::LinkedList;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let raw_input = fs::read_to_string(file_path).expect("Couldn't open file");

    let interesting_cycles = [20, 60, 100, 140, 180, 220];
    let mut instruction_stack = LinkedList::new();

    let mut register_value = 1;
    let mut signal_strength = 0;

    let instruction_lines: Vec<Vec<&str>> = raw_input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();
    let mut cycle = 1;

    // part two
    let mut screen = ['.'; 240];

    while cycle < 240 {
        if cycle - 1 < instruction_lines.len() {
            match instruction_lines[cycle - 1][0] {
                "addx" => {
                    let v = instruction_lines[cycle - 1][1].parse::<i32>().unwrap();
                    instruction_stack.push_front((2, v));
                }
                "noop" => {
                    instruction_stack.push_front((1, 0));
                }
                _ => eprintln!("Invalid input"),
            }
        }

        // part two
        let sprite = [register_value - 1, register_value, register_value + 1];
        if sprite.contains(&((cycle as i32 - 1) % 40)) {
            screen[cycle - 1] = '#';
        }

        cycle += 1;

        // part one
        if !instruction_stack.is_empty() {
            let instruction = instruction_stack.back_mut().unwrap();
            instruction.0 -= 1;
            if instruction.0 == 0 {
                register_value += instruction_stack.pop_back().unwrap().1;
            }
        }

        if interesting_cycles.contains(&cycle) {
            signal_strength += cycle as i32 * register_value;
        }
    }

    // part one
    println!("{signal_strength}");

    // part two
    for row_pixels in screen.chunks(40) {
        let row = row_pixels.iter().collect::<String>();
        println!("{row}");
    }
}
