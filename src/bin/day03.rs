use ::std::env;
use ::std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let inputs_raw = fs::read_to_string(file_path).expect("Couldn't open file");

    // traversing trough them at the same time
    // lowercase char - 97
    // uppercase char - 39
    let mut sum: i32 = 0;
    for line in inputs_raw.lines() {
        let mut first_items: [bool; 123] = [false; 123];
        let mut second_items: [bool; 123] = [false; 123];
        let first_compartment = &line.as_bytes()[..line.len() / 2];
        let second_compartment = &line.as_bytes()[line.len() / 2..];

        for i in 0..first_compartment.len() {
            let left = first_compartment[i];
            let right = second_compartment[i];
            first_items[left as usize] = true;
            second_items[right as usize] = true;

            if first_items[right as usize] == true {
                sum += (if right > 96 { right - 96 } else { right - 38 }) as i32;
                break;
            } else if second_items[left as usize] == true {
                sum += (if left > 96 { left - 96 } else { left - 38 }) as i32;
                break;
            }
        }
    }
    println!("{sum}");

    // part two
    let inputs: Vec<&str> = inputs_raw.split("\n").collect();

    let sum = inputs.chunks(3).fold(0, |acc, group| {
        let mut first_items: [bool; 123] = [false; 123];
        let mut second_items: [bool; 123] = [false; 123];
        let mut third_items: [bool; 123] = [false; 123];

        for i in 0..group.iter().map(|x| x.len()).max().unwrap() {
            let first_opt = if group[0].as_bytes().len() > i {
                first_items[group[0].as_bytes()[i] as usize] = true;
                Some(group[0].as_bytes()[i])
            } else {
                None
            };
            let second_opt = if group[1].as_bytes().len() > i {
                second_items[group[1].as_bytes()[i] as usize] = true;
                Some(group[1].as_bytes()[i])
            } else {
                None
            };
            let third_opt = if group[2].as_bytes().len() > i {
                third_items[group[2].as_bytes()[i] as usize] = true;
                Some(group[2].as_bytes()[i])
            } else {
                None
            };

            match (first_opt, second_opt, third_opt) {
                (Some(first), Some(second), Some(third)) => {
                    if first_items[second as usize] && third_items[second as usize] {
                        return acc + get_priority(second);
                    } else if second_items[first as usize] && third_items[first as usize] {
                        return acc + get_priority(first);
                    } else if second_items[third as usize] && first_items[third as usize] {
                        return acc + get_priority(third);
                    }
                }
                (Some(first), None, Some(third)) => {
                    if second_items[first as usize] && third_items[first as usize] {
                        return acc + get_priority(first);
                    } else if second_items[third as usize] && first_items[third as usize] {
                        return acc + get_priority(third);
                    }
                }
                (None, Some(second), Some(third)) => {
                    if first_items[second as usize] && third_items[second as usize] {
                        return acc + get_priority(second);
                    } else if second_items[third as usize] && first_items[third as usize] {
                        return acc + get_priority(third);
                    }
                }
                (None, None, Some(third)) => {
                    if second_items[third as usize] && first_items[third as usize] {
                        return acc + get_priority(third);
                    }
                }
                (None, Some(second), None) => {
                    if first_items[second as usize] && third_items[second as usize] {
                        return acc + get_priority(second);
                    }
                }
                (Some(first), None, None) => {
                    if second_items[first as usize] && third_items[first as usize] {
                        return acc + get_priority(first);
                    }
                }
                (Some(first), Some(second), None) => {
                    if first_items[second as usize] && third_items[second as usize] {
                        return acc + get_priority(second);
                    } else if second_items[first as usize] && third_items[first as usize] {
                        return acc + get_priority(first);
                    }
                }
                (None, None, None) => return acc,
            }
        }
        acc
    });
    println!("{sum}")
}

fn get_priority(ascii_number: u8) -> i32 {
    if ascii_number > 96 {
        return ascii_number as i32 - 96;
    } else {
        return ascii_number as i32 - 38;
    };
}
