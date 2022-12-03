use std::io;
use std::vec::Vec;
use std::collections::BTreeSet;



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

    let sum_badge_priorities = get_sum_badge_priorities(&input);
    println!("The sum of the priorities of each badge is {sum_badge_priorities}");
}

fn find_badge(group: &[String]) -> char {
    if group.len() != 3 {
	panic!("Group does not have 3 elves!");
    }

    let mut first_set = BTreeSet::<char>::new();
    let mut second_set = BTreeSet::<char>::new();
    let mut third_set = BTreeSet::<char>::new();

    let mut iter = group.iter();
    for item in iter.next().expect("Missing?").chars() {
	first_set.insert(item);
    }
    for item in iter.next().expect("Missing?").chars() {
	second_set.insert(item);
    }
    for item in iter.next().expect("Missing?").chars() {
	third_set.insert(item);
    }

    let first_intersection = first_set.intersection(&second_set).copied().collect::<BTreeSet<char>>();

    match first_intersection.intersection(&third_set).next() {
	Some(c) => *c,
	None => panic!("No badge found!")
    }
}

fn get_sum_badge_priorities(input: &Vec<String>) -> u32 {
    const LOWER_OFFSET: u32 = 'a' as u32 - 1;
    const UPPER_OFFSET: u32 = 'A' as u32 - 27;
    
    let mut sum_priorities = 0;

    for group in input.chunks(3) {
	let dupe_char_code = find_badge(group);
	sum_priorities += match dupe_char_code {
	    'a'..='z' => dupe_char_code as u32 - LOWER_OFFSET,
	    'A'..='Z' => dupe_char_code as u32 - UPPER_OFFSET,
	    _ => 0
	}
    }
    
    return sum_priorities;
}

fn _find_duplicate(rucksack: &String) -> char {
    let left_pocket = rucksack.get(..rucksack.len() / 2).unwrap();
    let right_pocket = rucksack.get(rucksack.len() / 2..).unwrap();

    let mut left_set = BTreeSet::<char>::new();
    let mut right_set = BTreeSet::<char>::new();

    for item in left_pocket.chars() {
	left_set.insert(item);
    }
    for item in right_pocket.chars() {
	right_set.insert(item);
    }
    
    return match left_set.intersection(&right_set).next() {
	Some(c) => *c,
	None => panic!("No duplicate found!")
    };
}

fn _get_sum_priorities(input: &Vec<String>) -> u32 {
    const LOWER_OFFSET: u32 = 'a' as u32 - 1;
    const UPPER_OFFSET: u32 = 'A' as u32 - 27;
    
    let mut sum_priorities = 0;

    for rucksack in input {
	let dupe_char_code = _find_duplicate(rucksack);
	sum_priorities += match dupe_char_code {
	    'a'..='z' => dupe_char_code as u32 - LOWER_OFFSET,
	    'A'..='Z' => dupe_char_code as u32 - UPPER_OFFSET,
	    _ => 0
	}
    }
    
    return sum_priorities;
}
