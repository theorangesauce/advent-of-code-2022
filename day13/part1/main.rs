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
		line = line.trim().to_string();
		input.push(line);
	    }
	    Err(_) => {
		break;
	    }
	}
    }

    let mut sum: usize = 0;
    for (idx, pair) in input.chunks(3).enumerate() {
	if check_packet_order(pair) {
	    sum += idx + 1;
	}
    }

    println!("The sum of the indices of ordered pairs is {sum}");
}

fn compare_packets(left_packet: &String, right_packet: &String) -> Option<bool> {
    let mut left_stack: Vec<usize> = Vec::new();
    let mut right_stack: Vec<usize> = Vec::new();

    let mut left_items: Vec<String> = Vec::new();
    let mut right_items: Vec<String> = Vec::new();
    
    for (idx, symbol) in left_packet.chars().enumerate() {
	if symbol == '[' {
	    left_stack.push(idx + 1);
	}
	else if symbol == ']' {
	    if left_stack.len() == 1 && idx > *left_stack.first().unwrap() {
		left_items.push(left_packet.get(left_stack.pop().unwrap()..idx).unwrap().to_string());
	    }
	    else {
		left_stack.pop();
	    }
	}
	else if symbol == ',' && left_stack.len() == 1 {
	    left_items.push(left_packet.get(left_stack.pop().unwrap()..idx).unwrap().to_string());
	    left_stack.push(idx + 1);
	}
    }

    for (idx, symbol) in right_packet.chars().enumerate() {
	if symbol == '[' {
	    right_stack.push(idx + 1);
	}
	else if symbol == ']' {
	    if right_stack.len() == 1 && idx > *right_stack.first().unwrap() {
		right_items.push(right_packet.get(right_stack.pop().unwrap()..idx).unwrap().to_string());
	    }
	    else {
		right_stack.pop();
	    }
	}
	else if symbol == ',' && right_stack.len() == 1 {
	    right_items.push(right_packet.get(right_stack.pop().unwrap()..idx).unwrap().to_string());
	    right_stack.push(idx + 1);
	}
    }
    
    for idx in 0..(left_items.len() + right_items.len()) {
	let left_item = match left_items.get(idx) {
	    Some(n) => n,
	    None => "_",
	};
	let right_item = match right_items.get(idx) {
	    Some(n) => n,
	    None => "_",
	};

	if left_item == "_" && right_item == "_" {
	    break;
	}
	else if left_item == "_" {
	    return Some(true);
	}
	else if right_item == "_" {
	    return Some(false);
	}
	else if !left_item.starts_with('[') && !right_item.starts_with('[') {
	    let left_val = left_item.parse::<i32>().expect("Bad left packet");
	    let right_val = right_item.parse::<i32>().expect("Bad right packet");

	    if left_val < right_val {
		//println!("left smaller, good!");
		return Some(true);
	    }
	    else if left_val > right_val {
		//println!("right smaller, bad!");
		return Some(false);
	    }
	}
	else {
	    let mut new_left_item = left_item.to_string();
	    let mut new_right_item = right_item.to_string();
	    
	    if !left_item.starts_with('[') {
		new_left_item = "[".to_owned() + left_item + "]";

	    }
	    else if !right_item.starts_with('[') {
		new_right_item = "[".to_owned() + right_item + "]"; 
	    }
	    
	    match compare_packets(&new_left_item, &new_right_item) {
		Some(n) => return Some(n),
		None => continue
	    }
	}
    }

    None
}

fn check_packet_order(pair: &[String]) -> bool {
    let left_packet = pair.get(0).unwrap();
    let right_packet = pair.get(1).unwrap();

    match compare_packets(left_packet, right_packet) {
	Some(true) => true,
	Some(false) => false,
	None => {
	    println!("Packets match?");
	    false
	}
    }
}

fn _print_packet_idxs(line: String) {
    let mut stack: Vec<usize> = Vec::new();

    for (idx, symbol) in line.chars().enumerate() {
	if symbol == '[' {
	    stack.push(idx);
	} 
	else if symbol == ']' {
	    let left_idx = stack.pop().unwrap();
	    println!("Packet from {left_idx}-{idx}");
	}
    }
    println!();
}
