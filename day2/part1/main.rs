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

    let score = calculate_guide_score(&input);

    println!("Following the guide will earn you {score} points.");
}

fn calculate_round_score(round: Vec<&str>) -> u32 {
    match round[..] {
	["A", "A"] => 4,
	["A", "B"] => 8,
	["A", "C"] => 3,
	["B", "A"] => 1,
	["B", "B"] => 5,
	["B", "C"] => 9,
	["C", "A"] => 7,
	["C", "B"] => 2,
	["C", "C"] => 6,
	_ => 0,
    }
}

fn calculate_guide_score(input: &Vec<String>) -> u32 {
    let mut score = 0;
    
    for line in input {
	let round = line.replace("X", "A").replace("Y", "B").replace("Z", "C");

	score += calculate_round_score(round.split(" ").collect());
    }

    return score;
}
