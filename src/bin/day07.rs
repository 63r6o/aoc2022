use ::std::env;
use ::std::fs;
use std::collections::HashMap;
use std::collections::LinkedList;

// one of the ugliest hacks I've ever produced.
// I'm pushing this to the repo as a sad reminder
// of what a sleep-deprived human being is capable of
// at 3 AM.

#[derive(Debug, PartialEq)]
enum File {
    Dir(String),
    File((String, u64)),
}

enum Command {
    ChangeToDir(String),
    GoBack,
    Ls,
    ListedDir(String),
    ListedFile((String, u64))
}


fn get_parent_name(path:  &LinkedList<String>) -> String {
    let mut path_copy = path.clone();
    let mut root = path_copy.pop_front().unwrap();
    let path_string = path_copy.iter().map(|x| x.clone()).collect::<Vec<String>>().join("/");
    root.push_str(&path_string);
    root
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let raw_inputs = fs::read_to_string(file_path).expect("Couldn't open file");

    let mut directories: HashMap<String, Vec<File>> = HashMap::new();
    let mut files: HashMap<String, u64> = HashMap::new();
    let mut directory_sizes: HashMap<String, u64> = HashMap::new();

    let mut history: LinkedList<String> = LinkedList::new();

    for line in raw_inputs.lines() {
        let terminal: Vec<&str> = line.split(' ').collect();

        let command = match terminal[0] {
            "$" => {
                if terminal[1] == "cd" {
                    if terminal[2] == ".." {
                        Command::GoBack
                    } else {
                        Command::ChangeToDir(terminal[2].to_string())
                    }
                } else {
                    Command::Ls
                }
            },
            "dir" => Command::ListedDir(terminal[1].to_string()),
            _ => Command::ListedFile((terminal[1].to_string(), terminal[0].parse::<u64>().unwrap())),
        };

        match command {
            Command::ChangeToDir(name) => {
                history.push_back(name.clone());

                let full_name = get_parent_name(&history);
                //dbg!(&full_name);
                if !directories.contains_key(&full_name) {
                    directories.insert(full_name, Vec::new());
                }
            },
            Command::GoBack => {
                history.pop_back();
            },
            Command::Ls => (),
            Command::ListedDir(name) => {
                let mut full_name = get_parent_name(&history);
                if full_name != "/" {
                    full_name.push_str("/");
                    full_name.push_str(&name);
                } else {
                    full_name.push_str(&name);
                }
                let dir = File::Dir(full_name.clone());
                if !directories.contains_key(&full_name) {
                    directories.insert(full_name, Vec::new());
                }

                let parent_name = get_parent_name(&history);

                let parent_contents = directories.get_mut(&parent_name).unwrap();
                if !parent_contents.contains(&dir) {
                    parent_contents.push(dir);
                }
            },
            Command::ListedFile((file_name, file_size)) => {
                let fi = File::File((file_name.clone(), file_size));
                if file_name == "fwh.wtt" {
                    dbg!("Helloooooo I'm here!!!!! I'm the file!!!");
                    let parent_name = get_parent_name(&history);
                    dbg!(parent_name);
                }
                if !files.contains_key(&file_name) {
                    files.insert(file_name, file_size);
                }

                let parent_name = get_parent_name(&history);
 
                let parent_contents = directories.get_mut(&parent_name).unwrap();
                if !parent_contents.contains(&fi) {
                    parent_contents.push(fi);
                }
            },
        }

    }


    let mut sizes: Vec<u64> = Vec::new();
    for (dir_name, contents) in directories.iter() {
        let mut size: u64 = 0;
        let mut stack = LinkedList::new();
        stack.push_back(dir_name);

        while !stack.is_empty() {
            let current_dir = stack.pop_back().unwrap();
            for file in directories.get(current_dir).unwrap() {
                match file {
                    File::Dir(name) => {
                        if directory_sizes.contains_key(name) {
                            size += directory_sizes.get(name).unwrap();
                        } else if !stack.contains(&name) {
                            stack.push_back(name);
                        }
                    },
                    File::File((file_name, file_size)) => {
                        size += file_size;
                    },
                }
            }
        }

        directory_sizes.insert(dir_name.to_string(), size);
        sizes.push(size);
        

    }

    let result_first: u64 = sizes.into_iter().filter(| size | size <= &(100000 as u64)).sum();
    println!("{result_first}");

    // part two
    // don't judge if you don't want to be judged
    let total_size = 70000000 as u64;
    let update_size = 30000000 as u64;
    let full_space = directory_sizes.get("/").unwrap();
    let free_space = total_size - *full_space;

    let needed_space = update_size - free_space;
    let result_second = directory_sizes.into_iter().filter(|(name, size)| {
        size >= &needed_space
    }).collect::<Vec<(String, u64)>>();
    let mut real_result = u64::MAX;
    for (_name, size) in result_second {
        if size < real_result {
            real_result = size;
        }
    }
    
    println!("{real_result}");
    
}

