use ::std::env;
use ::std::fs;

enum From {
    Left,
    Right,
    Top,
    Bottom,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let raw_input = fs::read_to_string(file_path).expect("Couldn't open file");

    let map = raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|height_string| height_string.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let visible_sides = (map[0].len() * 2) + (map.len() * 2 - 4);
    let mut visible_interior = 0;

    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            let tree = map[i][j];
            if is_visible(From::Left, tree, (i as i32, j as i32), &map)
                || is_visible(From::Right, tree, (i as i32, j as i32), &map)
                || is_visible(From::Top, tree, (i as i32, j as i32), &map)
                || is_visible(From::Bottom, tree, (i as i32, j as i32), &map)
            {
                visible_interior += 1;
            }
        }
    }

    let visible_sum = visible_sides + visible_interior;
    println!("{visible_sum}");

    //part two
    let mut scenic_score = 0;

    for i in 0..map.len() - 1 {
        for j in 0..map[i].len() - 1 {
            let tree = map[i][j];
            let left_score = get_viewing_distance(From::Left, tree, (i as i32, j as i32), &map);
            let right_score = get_viewing_distance(From::Right, tree, (i as i32, j as i32), &map);
            let top_score = get_viewing_distance(From::Top, tree, (i as i32, j as i32), &map);
            let bottom_score = get_viewing_distance(From::Bottom, tree, (i as i32, j as i32), &map);

            let tree_scenic_score = left_score * right_score * top_score * bottom_score;
            if tree_scenic_score > scenic_score {
                scenic_score = tree_scenic_score;
            }
        }
    }

    println!("{scenic_score}");
}

fn get_viewing_distance(
    from: From,
    tree: u32,
    (mut i, mut j): (i32, i32),
    map: &Vec<Vec<u32>>,
) -> i32 {
    let mut viewing_distance = 0;
    match from {
        From::Left => {
            j -= 1;
            while j > -1 {
                viewing_distance += 1;
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return viewing_distance;
                }
                j -= 1;
            }
            viewing_distance
        }
        From::Right => {
            j += 1;
            while j < map[i as usize].len().try_into().unwrap() {
                viewing_distance += 1;
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return viewing_distance;
                }
                j += 1
            }
            viewing_distance
        }
        From::Top => {
            i -= 1;
            while i > -1 {
                viewing_distance += 1;
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return viewing_distance;
                }
                i -= 1
            }
            viewing_distance
        }
        From::Bottom => {
            i += 1;
            while i < map.len().try_into().unwrap() {
                viewing_distance += 1;
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return viewing_distance;
                }
                i += 1
            }
            viewing_distance
        }
    }
}

fn is_visible(from: From, tree: u32, (mut i, mut j): (i32, i32), map: &Vec<Vec<u32>>) -> bool {
    match from {
        From::Left => {
            j -= 1;
            while j > -1 {
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return false;
                }
                j -= 1;
            }
            true
        }
        From::Right => {
            j += 1;
            while j < map[i as usize].len().try_into().unwrap() {
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return false;
                }
                j += 1
            }
            true
        }
        From::Top => {
            i -= 1;
            while i > -1 {
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return false;
                }
                i -= 1
            }
            true
        }
        From::Bottom => {
            i += 1;
            while i < map.len().try_into().unwrap() {
                let neighbour = map[i as usize][j as usize];
                if neighbour >= tree {
                    return false;
                }
                i += 1
            }
            true
        }
    }
}
