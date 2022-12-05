use ::std::env;
use ::std::fs;
use std::collections::LinkedList;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let inputs_raw = fs::read_to_string(file_path).expect("Couldn't open file");

    // 3 characters -> crates or empty
    // 1 character -> divider
    // number of stacks -> (length of the input line / 4) + 1
    let inputs: Vec<&str> = inputs_raw.split("\n\n").collect();

    let stack_drawings: Vec<Vec<char>> = inputs[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let moves: Vec<Vec<usize>> = inputs[1]
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|item| {
                    if item.parse::<usize>().is_ok() {
                        Some(item.parse::<usize>().unwrap())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let num_of_stacks = stack_drawings[0].len() / 4 + 1;

    let mut stacks_old_crane: Vec<LinkedList<char>> =
        (0..num_of_stacks).map(|_| LinkedList::new()).collect();
    let mut stacks_new_crane: Vec<LinkedList<char>> =
        (0..num_of_stacks).map(|_| LinkedList::new()).collect();

    for line in stack_drawings {
        line.chunks(4).enumerate().for_each(|(i, chunk)| {
            if chunk[1].is_alphabetic() {
                stacks_old_crane[i].push_front(chunk[1]);
                stacks_new_crane[i].push_front(chunk[1])
            }
        });
    }

    // part 1
    moves.iter().for_each(|line| {
        let range = line[0];
        let from = line[1] - 1;
        let to = line[2] - 1;

        for _i in 0..range {
            let item = stacks_old_crane[from].pop_back().unwrap();
            stacks_old_crane[to].push_back(item)
        }
    });

    // part 2
    moves.iter().for_each(|line| {
        let range = line[0];
        let from = line[1] - 1;
        let to = line[2] - 1;

        let mut items: LinkedList<char> = LinkedList::new();

        for _i in 0..range {
            let item = stacks_new_crane[from].pop_back().unwrap();
            items.push_front(item);
        }

        for i in items {
            stacks_new_crane[to].push_back(i)
        }
    });

    let first_result: String = stacks_old_crane
        .iter()
        .map(|line| line.back().unwrap_or(&' '))
        .collect();
    let second_result: String = stacks_new_crane
        .iter()
        .map(|line| line.back().unwrap_or(&' '))
        .collect();
    println!("{first_result}");
    println!("{second_result}");
}
