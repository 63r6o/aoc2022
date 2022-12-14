use ::std::env;
use ::std::fs;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Multiple(Option<u64>),
    Add(u64),
}
#[derive(Debug)]
struct Monkey {
    items: RefCell<VecDeque<u64>>,
    operation: Operation,
    test_prime: u64,
    true_monkey_index: usize,
    false_monkey_index: usize,
    number_of_inspections: Cell<u64>,
}

impl Monkey {
    pub fn new(
        items: VecDeque<u64>,
        operation: Operation,
        test_prime: u64,
        true_monkey_index: usize,
        false_monkey_index: usize,
    ) -> Monkey {
        Monkey {
            items: RefCell::new(items),
            operation,
            test_prime,
            true_monkey_index,
            false_monkey_index,
            number_of_inspections: Cell::new(0),
        }
    }

    fn operate(&self, item: u64) -> u64 {
        match self.operation {
            Operation::Multiple(value_option) => match value_option {
                Some(value) => item * value,
                None => item * item,
            },
            Operation::Add(value) => item + value,
        }
    }

    fn test(&self, item: u64) -> usize {
        if item % self.test_prime == 0 {
            self.true_monkey_index
        } else {
            self.false_monkey_index
        }
    }

    fn inspect_items(&self, test_prime_products: Option<u64>) -> VecDeque<(usize, u64)> {
        let mut items_to_throw: VecDeque<(usize, u64)> = VecDeque::new();
        loop {
            let item_option = self.items.borrow_mut().pop_front();
            match item_option {
                Some(mut item) => {
                    let number_of_inspections = self.number_of_inspections.get() + 1;
                    self.number_of_inspections.set(number_of_inspections);

                    item = match test_prime_products {
                        // part two
                        Some(prime_product) => self.operate(item) % prime_product,
                        // part one
                        None => self.operate(item) / 3,
                    };

                    let monkey_index = self.test(item);
                    items_to_throw.push_back((monkey_index, item));
                }
                None => return items_to_throw,
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let raw_input = fs::read_to_string(file_path).expect("Couldn't open file");

    let mut monkeys: Vec<Monkey> = Vec::new();

    // parse input
    for note in raw_input.split("\n\n") {
        let note_lines: Vec<&str> = note.split('\n').collect();
        let items: VecDeque<u64> = note_lines[1].split(": ").collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|item_string| item_string.parse::<u64>().unwrap())
            .collect();

        let operation_description: Vec<&str> = note_lines[2].split(" = ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect();

        let operation = match (
            operation_description[0],
            operation_description[1],
            operation_description[2].parse::<u64>(),
        ) {
            (_, "+", Ok(number)) => Operation::Add(number),
            (_, "*", Ok(number)) => Operation::Multiple(Some(number)),
            _ => Operation::Multiple(None),
        };

        let test_prime = note_lines[3]
            .split(' ')
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let true_monkey_index = note_lines[4]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_monkey_index = note_lines[5]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let monkey = Monkey::new(
            items,
            operation,
            test_prime,
            true_monkey_index,
            false_monkey_index,
        );

        monkeys.push(monkey)
    }

    // part one
    // for _ in 0..20 {
    //     monkeys.iter().for_each(|monkey| {
    //         let items_to_throw = monkey.inspect_items(None);
    //         for (monkey_index, item) in items_to_throw {
    //             monkeys
    //                 .get(monkey_index)
    //                 .unwrap()
    //                 .items
    //                 .borrow_mut()
    //                 .push_back(item);
    //         }
    //     });
    // }

    // Chinese remainder theorem
    // https://brilliant.org/wiki/chinese-remainder-theorem/
    // I had to look it up
    let test_prime_products = monkeys
        .iter()
        .map(|monkey| monkey.test_prime)
        .product::<u64>();

    for _ in 0..10000 {
        monkeys.iter().for_each(|monkey| {
            let items_to_throw = monkey.inspect_items(Some(test_prime_products));
            for (monkey_index, item) in items_to_throw {
                monkeys
                    .get(monkey_index)
                    .unwrap()
                    .items
                    .borrow_mut()
                    .push_back(item);
            }
        });
    }

    let mut number_of_inspections: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.number_of_inspections.get())
        .collect();

    number_of_inspections.sort_by(|a, b| b.cmp(a));

    let monkey_business = number_of_inspections.iter().take(2).product::<u64>();
    println!("{monkey_business}");
}
