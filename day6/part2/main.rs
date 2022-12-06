use std::io;
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

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

    for line in input {
	let message_idx = find_first_message(line).expect("No message marker found!");
	println!("The first message marker ends after {message_idx} characters");
    }
}

fn _find_first_packet(line: String) -> Option<usize> {
    let mut idx = 0;
    let mut marker = VecDeque::<char>::new();

    for char in line.chars() {
	idx += 1;
	marker.push_back(char);
	while marker.len() > 4 {
	    marker.pop_front();
	}
	if HashSet::<char>::from_iter(marker.iter().cloned()).len() == 4 {
	    return Some(idx);
	}
    }

    return None;
}

fn find_first_message(line: String) -> Option<usize> {
    let mut idx = 0;
    let mut marker = VecDeque::<char>::new();

    for char in line.chars() {
	idx += 1;
	marker.push_back(char);
	while marker.len() > 14 {
	    marker.pop_front();
	}
	if HashSet::<char>::from_iter(marker.iter().cloned()).len() == 14 {
	    return Some(idx);
	}
    }

    return None;
}
