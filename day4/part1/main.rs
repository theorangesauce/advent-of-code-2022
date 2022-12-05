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

    let overlap_count = get_overlap_count(&input);

    println!("{overlap_count} pairs of elves have overlapping sections");
}

fn get_overlap_count(input: &Vec<String>) -> u32 {
    let mut overlap_count = 0;
    for line in input {
	if is_overlapping(line.split(',').collect()) {
	    overlap_count += 1;
	}
    }

    return overlap_count;
}

fn is_overlapping(line: Vec<&str>) -> bool {
    let first_range: Vec<u32> =
	line.first().unwrap()
	.split("-")
	.map(|range| range.parse::<u32>().unwrap())
	.collect();
    let second_range: Vec<u32> =
	line.last().unwrap()
	.split("-")
	.map(|range| range.parse::<u32>().unwrap())
	.collect();

    if first_range.first() <= second_range.first() && first_range.last() >= second_range.last()
	|| second_range.first() <= first_range.first() && second_range.last() >= first_range.last() {
	    return true;
	}
    
    return false;
}


