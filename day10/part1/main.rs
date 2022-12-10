use std::io;
use std::vec::Vec;

struct CPU {
    register: i64,
    value_to_add: i64,
    is_adding: bool,
}

impl CPU {
    fn init() -> CPU {
	return CPU {
	    register: 1,
	    value_to_add: 0,
	    is_adding: false,
	}
    }

    fn get_register(&self) -> i64 {
	return self.register;
    }
    
    fn tick(&mut self, input_iter: &mut dyn Iterator<Item = &String>) {
	if self.is_adding {
	    self.register += self.value_to_add;
	    self.value_to_add = 0;
	    self.is_adding = false;
	}
	else {
	    match input_iter.next() {
		Some(instruction) => {
		    match instruction.chars().nth(0).unwrap() {
			'n' => return,
			'a' => {
			    self.value_to_add = instruction.split(' ').nth(1).expect("Invalid value")
				.parse::<i64>().expect("Failed to parse");
			    self.is_adding = true;
			},
			_ => println!("Invalid instruction"),
		    }
		},
		None => return,
	    }
	}
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

    let mut cpu = CPU::init();
    let mut input_iter = input.iter();

    let mut signal_strength_sum = 0;
    for count in 1..=220 {
	cpu.tick(&mut input_iter);
	if (count + 20) % 40 == 0 {
	    signal_strength_sum += count * cpu.get_register();
	}
    }

    println!("The sum of the signal strengths is {}", signal_strength_sum);
}
