use std::io;
use std::vec::Vec;
use std::collections::BTreeSet;

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

    let three_most_calories = find_three_most_calories(input);
    println!("The three elves with the most calories are holding {three_most_calories} calories of food.");
}

fn find_three_most_calories(input: Vec<String>) -> u32 {
    let mut set = BTreeSet::new();
    let mut three_most_calories: u32 = 0;

    let elves = input.split(|line| line == "");
    for elf in elves {
	let mut calories = 0;
	for item in elf.iter() {
	    calories += item.parse::<u32>().expect("Failed to parse int?");
	}
	
	set.insert(calories);
    }

    let mut iter = set.iter();
    for _ in 0..3 {
	three_most_calories += match iter.next_back() {
	    None => 0,
	    Some(calories) => *calories,
	};
    }

    return three_most_calories;
}


fn _find_most_calories(input: Vec<String>) -> u32 {
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
