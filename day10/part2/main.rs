use std::io;
use std::vec::Vec;

struct CPU {
    cycle: u64,
    register: i64,
    value_to_add: i64,
    is_adding: bool,
}

impl CPU {
    fn init() -> CPU {
	return CPU {
	    cycle: 0,
	    register: 1,
	    value_to_add: 0,
	    is_adding: false,
	}
    }

    fn _get_register(&self) -> i64 {
	return self.register;
    }

    fn draw(&self) {
	if (self.cycle % 40) as i64 >= self.register - 1 && (self.cycle % 40) as i64 <= self.register + 1 {
	    print!("#");
	} else {
	    print!(".");
	}
	if (self.cycle + 1) % 40 == 0 {
	    println!();
	}
    }
    
    fn tick(&mut self, input_iter: &mut dyn Iterator<Item = &String>) {
	self.draw();
	self.cycle += 1;
	
	if self.is_adding {
	    self.register += self.value_to_add;
	    self.value_to_add = 0;
	    self.is_adding = false;
	}
	else {
	    match input_iter.next() {
		Some(instruction) => {
		    match instruction.chars().nth(0).unwrap() {
			'n' => (),
			'a' => {
			    self.value_to_add = instruction.split(' ').nth(1).expect("Invalid value")
				.parse::<i64>().expect("Failed to parse");
			    self.is_adding = true;
			},
			_ => println!("Invalid instruction"),
		    }
		},
		None => (),
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

    for _ in 1..=240 {
	cpu.tick(&mut input_iter);
    }
}
