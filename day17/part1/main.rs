use std::io;
use std::collections::HashSet;
use std::collections::VecDeque;
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

    let jet_pattern: VecDeque<char> = input.last().unwrap().chars().collect::<VecDeque<char>>();

    let highest_rock = drop_rocks(jet_pattern);

    println!("The highest rock is at y = {highest_rock}");

}

fn drop_rocks(mut jet_pattern: VecDeque<char>) -> usize {
    let block_sets = generate_block_sets();
    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut current_block: usize = 0;
    let mut highest_point: usize = 0;

    for _ in 0..2022 {
	let mut y_offset = highest_point + 4;
	let mut x_offset = 3;
	
	loop {
	    // apply jet
	    let jet: char = jet_pattern.pop_front().unwrap();
	    match jet {
		'<' => {
		    x_offset -= 1;
		    for point in block_sets.get(current_block).unwrap().iter() {
			if rocks.contains(&(point.0 + y_offset, point.1 + x_offset,))
			    || (point.1 + x_offset) % 8 == 0
			{
			    x_offset += 1;
			}
		    }
		},
		'>' => {
		    x_offset += 1;
		    for point in block_sets.get(current_block).unwrap().iter() {
			if rocks.contains(&(point.0 + y_offset, point.1 + x_offset,))
			    || (point.1 + x_offset) % 8 == 0
			{
			    x_offset -= 1;
			}
		    }
		},
		_ => {},
	    }
	    jet_pattern.push_back(jet);
	    
	    // drop block
	    let mut stopped = false;
	    y_offset -= 1;
	    for point in block_sets.get(current_block).unwrap().iter() {
		if rocks.contains(&(point.0 + y_offset, point.1 + x_offset))
		    || point.0 + y_offset == 0
		{
		    stopped = true;
		    y_offset += 1;
		}
	    }

	    if stopped {
		for point in block_sets.get(current_block).unwrap().iter() {
		    if point.0 + y_offset > highest_point {
			highest_point = point.0 + y_offset;
		    }
		    rocks.insert((point.0 + y_offset, point.1 + x_offset));
		}
		break;
	    }
	}
	
	current_block = (current_block + 1) % block_sets.len();
    }

    highest_point
}

fn generate_block_sets() -> Vec<HashSet<(usize, usize)>> {
    let mut block_sets = Vec::new();

    block_sets.push(HashSet::from([(0,0), (0,1), (0,2), (0,3)]));
    block_sets.push(HashSet::from([(0,1), (1,0), (1,1), (1,2), (2,1)]));
    block_sets.push(HashSet::from([(0,0), (0,1), (0,2), (1,2), (2,2)]));
    block_sets.push(HashSet::from([(0,0), (1,0), (2,0), (3,0)]));
    block_sets.push(HashSet::from([(0,0), (0,1), (1,0), (1,1)]));
    block_sets
}
