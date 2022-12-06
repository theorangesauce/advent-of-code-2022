use std::io;
use std::vec::Vec;

fn main() {
    let mut input = Vec::new();

    loop {
	let mut line = String::new();
	
	match io::stdin().read_line(&mut line) {
	    Ok(n) => {
		if n == 0 {
		    break;
		}
		line = line.trim_end().to_string();
		input.push(line);
	    }
	    Err(_) => {
		break;
	    }
	}
    }

    let mut input_split = input.split(|line| line == "");
    let cargo_input = input_split.next().unwrap();
    let action_input = input_split.next().unwrap();
    
    let mut stacks: Vec<Vec<char>> = create_stacks(&cargo_input);

    print_stack_state(&stacks);

    for operation in action_input {
	apply_operation(&mut stacks, &operation);
    }

    let mut top_crates = String::new();

    for stack in &stacks {
	top_crates.push(match stack.last() {
	    Some(c) => *c,
	    None => '_'
	})
    }
    println!("The crates on top of each stack are: {top_crates}");
}

fn apply_operation(stacks: &mut Vec<Vec<char>>, operation: &String) {
    let mut line_split = operation.split(' ');

    let count = line_split.nth(1).unwrap().parse::<usize>().expect("Failed to parse count!");
    let stack1_idx = line_split.nth(1).unwrap().parse::<usize>().expect("Failed to parse stack 1 index!") - 1;
    let stack2_idx = line_split.nth(1).unwrap().parse::<usize>().expect("Failed to parse stack 2 index!") - 1;

    let mut moved_chars = Vec::<char>::new();

    for _ in 0..count {
	match stacks.get_mut(stack1_idx).expect("No stack at index 1!").pop() {
	    Some(c) => {
		moved_chars.push(c);
	    },
	    None => break,
	}
    }

    moved_chars.reverse();
    
    for c in moved_chars {
	stacks.get_mut(stack2_idx).expect("No stack at index 2!").push(c);
    }
}

fn create_stacks(cargo_input: &[String]) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut input_iter = cargo_input.iter().rev();

    for _ in 0..get_num_columns(input_iter.next().expect("Failed to parse column numbers!")) {
	stacks.push(Vec::new());
    }

    loop {
	match input_iter.next() {
	    Some(line) => {
		for i in 0..stacks.len() {
		    let idx = (i + 1) * 4 - 3;
		    let substr = line.get(idx..(idx + 1));

		    match substr {
			Some(c) => {
			    let item = c.chars().next().unwrap();
			    if item != ' ' {
				stacks.get_mut(i).unwrap().push(item);
			    }
			},
			_ => continue
		    }
		}
	    },
	    None => break,
	}
    }

    return stacks;
}
    
fn get_num_columns(line: &String) -> usize {
    return line.replace(' ', "").len();
}

fn print_stack_state(stacks: &Vec<Vec<char>>) {
    for i in stacks {
	for k in i {
	    print!("{k}");
	}
	println!("");
    }
    println!("");
}
    
