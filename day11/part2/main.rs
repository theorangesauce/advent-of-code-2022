use std::io;
use std::vec::Vec;

struct Monkey {
    items: Vec<i64>,
    operation_type: char,
    operation_value: i64,
    test_value: i64,
    true_dest: usize,
    false_dest: usize,
    items_investigated: u64,
}

struct ValueDest(i64, usize);

impl Monkey {
    fn do_turn(&mut self) -> Vec<ValueDest> {
	let mut values_to_move: Vec<ValueDest> = Vec::new();
	
	for item in &self.items {
	    let value = self.do_operation(*item);
	    let dest = self.test_value(value);
	    
	    values_to_move.push(ValueDest(value, dest));
	    self.items_investigated += 1;
	}

	self.items.clear();
	
	return values_to_move;
    }

    fn do_operation(&self, value: i64) -> i64 {
	match self.operation_type {
	    '+' => value + self.operation_value,
	    '*' => value * self.operation_value,
	    '2' => value * value,
	    _ => value,
	}
    }

    fn test_value(&self, value: i64) -> usize {
	if value % self.test_value == 0 {
	    return self.true_dest;
	}
	else {
	    return self.false_dest;
	}
    }
    
    fn _print_monkey(&self) {
	print!("Holding {{");
	for item in &self.items {
	    print!(" {item},");
	}
	println!(" }}");

	println!("operation: old {} {}", self.operation_type, self.operation_value);

	println!("check if mod {}", self.test_value);
	println!("monkey {} if true, monkey {} if false", self.true_dest, self.false_dest);
    }
}

fn main() {
    let mut input = Vec::new();

    loop {
	let mut line = String::new();
	
	match io::stdin().read_line(&mut line) {
	    Ok(n) => {
		if n == 0 {
		    break;
		}
		line = line.trim().to_string();
		input.push(line);
	    }
	    Err(_) => {
		break;
	    }
	}
    }
    let mut monkeys: Vec<Monkey> = Vec::new();
    let input_split = input.chunks(7);

    for monkey_input in input_split {
	monkeys.push(create_monkey(monkey_input));
    }
    
    let mut item_queue: Vec<Vec<i64>> = Vec::new();
    let mut mod_value: i64 = 1;
    
    for idx in 0..monkeys.len() {
	item_queue.push(Vec::new());
	mod_value *= monkeys.get(idx).unwrap().test_value;
    }
    
    for _ in 0..10000 {
	for (idx, monkey) in monkeys.iter_mut().enumerate() {
	    for item in item_queue.get(idx).unwrap() {
		monkey.items.push(*item);
	    }
	    item_queue.get_mut(idx).unwrap().clear();
	    
	    let values_to_move = monkey.do_turn();

	    for pair in values_to_move {
		item_queue.get_mut(pair.1).expect("No monkey at index")
		    .push(pair.0 % mod_value);
	    }
	}
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.items_investigated);
    monkeys.reverse();

    let mut business_level = monkeys.get(0).unwrap().items_investigated;
    business_level *= monkeys.get(1).unwrap().items_investigated;

    println!("The level of monkey business encountered was {business_level}");
}

fn create_monkey(monkey_input: &[String]) -> Monkey {
    let mut lines = monkey_input.iter();
    lines.next();

    // items
    let items = lines.next().unwrap()
	.strip_prefix("Starting items: ").expect("Malformed input (items)")
	.split(", ")
	.map(|item| item.parse::<i64>().expect("Bad item"))
	.collect::<Vec<i64>>();

    // operation
    let operation_line = lines.next().unwrap()
	.strip_prefix("Operation: new = old ").expect("Malformed input (operation)");
    
    let (operation_type, operation_value) = match operation_line.find("old") {
	Some(_) => ('2', 0),
	None => {
	    let op_type = operation_line.chars().nth(0).unwrap();
	    let op_value = operation_line.get(2..).expect("Malformed input (operation value)")
		.parse::<i64>().expect("Malformed input (operation value)");

	    (op_type, op_value)
	},
    };

    // divisibility test
    let test_value = lines.next().unwrap()
	.strip_prefix("Test: divisible by ").expect("Malformed input (test value)")
	.parse::<i64>().expect("Malformed input (operation value)");

    // passers
    let true_dest = lines.next().unwrap()
	.strip_prefix("If true: throw to monkey ").expect("Malformed input (true throw)")
	.parse::<usize>().expect("Malformed input (true throw)");
    
    let false_dest = lines.next().unwrap()
	.strip_prefix("If false: throw to monkey ").expect("Malformed input (false throw)")
	.parse::<usize>().expect("Malformed input (false throw)");
    
    return Monkey {
	items: items,
	operation_type: operation_type,
	operation_value: operation_value,
	test_value: test_value,
	true_dest: true_dest,
	false_dest: false_dest,
	items_investigated: 0
    };
}
