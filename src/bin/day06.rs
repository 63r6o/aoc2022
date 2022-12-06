use ::std::env;
use ::std::fs;
use std::collections::HashSet;

fn get_marker(datastream: &[char], window_size: usize) -> usize {
    for (i, signal) in datastream.windows(window_size).enumerate() {
        let mut unique = HashSet::new();
        if signal.iter().all(|j| unique.insert(j)) {
            return i + window_size;
        }
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let raw_inputs = fs::read_to_string(file_path).expect("Couldn't open file");

    let inputs: Vec<char> = raw_inputs.chars().collect();

    let start_of_packet = get_marker(&inputs, 4);
    let start_of_message = get_marker(&inputs, 14);
    println!("{start_of_packet}");
    println!("{start_of_message}")
}
