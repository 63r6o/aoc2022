use ::std::env;
use ::std::fs;
use std::cell::RefCell;
use std::collections::LinkedList;

#[derive(Debug)]
struct Node {
    id: usize,
    elevation: i8,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: RefCell<Vec<Vec<usize>>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: RefCell::new(Vec::new()),
        }
    }

    pub fn add_node(&mut self, elevation: i8) {
        let node = Node {
            id: self.nodes.len(),
            elevation,
        };
        self.nodes.push(node);
        self.edges.borrow_mut().push(Vec::new())
    }

    pub fn add_edge(&self, from: &Node, to: &Node) {
        if Graph::is_valid_step(from, to) {
            self.edges.borrow_mut()[from.id].push(to.id)
        }
    }

    fn is_valid_step(from: &Node, to: &Node) -> bool {
        to.elevation - from.elevation <= 1
    }

    fn bfs(
        &self,
        from: usize,
        destination: usize,
        predecessor: &mut [i32],
        distance: &mut [i32],
    ) -> bool {
        let mut visited: Vec<bool> = Vec::new();
        self.nodes.iter().for_each(|_| visited.push(false));

        let mut queue = LinkedList::new();
        visited[from] = true;
        distance[from] = 0;
        queue.push_front(from);

        while !queue.is_empty() {
            let current_id = queue.pop_front().unwrap();

            for node in &self.edges.borrow()[current_id] {
                if !visited[*node] {
                    visited[*node] = true;
                    queue.push_back(*node);
                    distance[*node] = distance[current_id] + 1;
                    predecessor[*node] = current_id as i32;

                    if *node == destination {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_shortest_path(&self, from: usize, destination: usize) -> Vec<usize> {
        let mut distance: Vec<i32> = Vec::new();
        self.nodes.iter().for_each(|_| distance.push(i32::MAX));

        let mut predecessor: Vec<i32> = Vec::new();
        self.nodes.iter().for_each(|_| predecessor.push(-1));

        if !self.bfs(from, destination, &mut predecessor, &mut distance) {
            //println!("There is no way to get there!")
        }

        let mut path = Vec::new();
        let mut crawl = destination;
        path.push(destination);

        while predecessor[crawl] != -1 {
            path.push(predecessor[crawl] as usize);
            crawl = predecessor[crawl] as usize;
        }
        path
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let raw_input = fs::read_to_string(file_path).expect("Couldn't open file");

    let map_width = raw_input.find('\n').unwrap();
    let mut start_index = 0;
    let mut destination_index = 0;

    let heightmap: Vec<i8> = raw_input
        .split('\n')
        .flat_map(|x| x.chars())
        .enumerate()
        .map(|(i, height_char)| {
            if height_char == 'S' {
                start_index = i;
                'a' as i8
            } else if height_char == 'E' {
                destination_index = i;
                'z' as i8
            } else {
                height_char as i8
            }
        })
        .collect();

    let mut graph = Graph::new();

    heightmap.iter().for_each(|height| {
        graph.add_node(*height);
    });

    for (i, _) in heightmap.iter().enumerate() {
        let from = &graph.nodes[i];

        if i + map_width < heightmap.len() {
            let to = &graph.nodes[i + map_width];
            graph.add_edge(from, to);
        }
        if i >= map_width {
            let to = &graph.nodes[i - map_width];
            graph.add_edge(from, to);
        }
        if i % map_width != 0 {
            let to = &graph.nodes[i - 1];
            graph.add_edge(from, to);
        }
        if i % map_width != map_width - 1 {
            let to = &graph.nodes[i + 1];
            graph.add_edge(from, to);
        }
    }

    // part one

    let path = graph.get_shortest_path(start_index, destination_index);
    let result_first = path.len() - 1;
    println!("{result_first}");

    // part two

    let result_second = heightmap
        .into_iter()
        .enumerate()
        .filter_map(
            |(i, height)| {
                if height == b'a' as i8 {
                    Some(i)
                } else {
                    None
                }
            },
        )
        .map(|start_index| {
            graph
                .get_shortest_path(start_index, destination_index)
                .len()
                - 1
        })
        .filter(|steps| *steps != 0)
        .min()
        .unwrap();
    println!("{result_second}");
}
