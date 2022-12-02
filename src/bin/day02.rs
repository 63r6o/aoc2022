use::std::collections::HashMap;
use::std::fs;
use std::env;

fn result(opponents_play: i32, my_play: i32, shapes: &[i32; 3]) -> i32 {
    if opponents_play == my_play { return 3}
    else if opponents_play == shapes[((my_play + 1) % 3) as usize] { return 6}
    else { return 0 }
}

// for part two
fn get_correct_step(opponents_play: i32, desired_outcome: i32, shapes: &[i32; 3]) -> i32 {
    if desired_outcome == 0 { return shapes[((opponents_play + 1) % 3) as usize]}
    else if desired_outcome == 3 { return opponents_play }
    else { return shapes[(opponents_play % 3) as usize] }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let inputs_raw = fs::read_to_string(file_path).expect("Couldn't open file");

    // Rock = 1, Paper = 2, Scissors = 3
    // points + 1 % 3 = which it can beat
    // points % 3 = which beats it
    let shapes = [1,2,3];
    
    let shape_points = HashMap::from([
        ("A",1),
        ("X",1),
        ("B",2),
        ("Y",2),
        ("C",3),
        ("Z",3)
    ]);

    let score = inputs_raw.lines().fold(0, | score, line | {
        let play: Vec<&str> = line.split(" ").collect();
        let opponents_play = shape_points.get(play[0]).unwrap();
        let my_play = shape_points.get(play[1]).unwrap();
        score + shape_points.get(play[1]).unwrap() + result(*opponents_play, *my_play, &shapes)
    });
    println!("{score}");

    // part two
    let desired_outcome_points = HashMap::from([
        ("X",0),
        ("Y",3),
        ("Z",6)
    ]);

    let score = inputs_raw.lines().fold(0, | score, line | {
        let play: Vec<&str> = line.split(" ").collect();
        let opponents_play = shape_points.get(play[0]).unwrap();
        let desired_outcome = desired_outcome_points.get(play[1]).unwrap();
        score + desired_outcome + get_correct_step(*opponents_play, *desired_outcome, &shapes)
    });
    println!("{score}");
}
