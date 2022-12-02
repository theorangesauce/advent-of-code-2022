use std::io;
use std::vec::Vec;

fn main() {
    let mut input = Vec::<String>::new();

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

    for line in &input {
	println!("{line}");
    }

    let most_calories = find_most_calories(input);

    println!("The elf with the most calories is holding {most_calories} calories worth");
}

fn find_most_calories(input: Vec<String>) -> u32 {
    let mut most_calories = 0;

    let elves = input.split(|line| line == "");
    for elf in elves {
	let mut calories = 0;
	for item in elf.iter() {
	    calories += item.parse::<u32>().expect("Failed to parse int?");
	}

	if calories > most_calories {
	    most_calories = calories;
	}
    }

    return most_calories;
}
