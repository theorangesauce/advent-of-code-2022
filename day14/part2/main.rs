use std::io;
use std::vec::Vec;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
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

    let mut cave_map: HashMap<Point, char> = HashMap::new();

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    
    for line in input {
	let (new_x, new_y) = add_blocks(&mut cave_map, line);

	if new_x > max_x {
	    max_x = new_x;
	}
	if new_y > max_y {
	    max_y = new_y;
	}
    }

    let starting_size = cave_map.len();

    loop {
	let current_size = cave_map.len();
	add_sand_unit(&mut cave_map, max_x, max_y);

	if current_size == cave_map.len() {
	    break;
	}
    }

    println!("{} sand units were added to the cave", cave_map.len() - starting_size);
}

fn add_sand_unit(cave_map: &mut HashMap<Point, char>, max_x: usize, max_y: usize) {
    let mut current_location = Point {x: 500, y: 0};

    if cave_map.contains_key(&current_location) {
	return;
    }
    
    loop {
	if current_location.y == max_y + 1 {
	    cave_map.insert(current_location, 'o');
	    break;
	}
	else if !cave_map.contains_key(&Point {x: current_location.x, y: current_location.y + 1}) {
	    current_location.y += 1;
	}
	else if current_location.x == 0 {
	    break;
	}
	else if !cave_map.contains_key(&Point {x: current_location.x - 1, y: current_location.y + 1}) {
	    current_location.x -= 1;
	    current_location.y += 1;
	}
	else if !cave_map.contains_key(&Point {x: current_location.x + 1, y: current_location.y + 1}) {
	    current_location.x += 1;
	    current_location.y += 1;
	}
	else {
	    cave_map.insert(current_location, 'o');
	    break;
	}
    }
}

fn add_blocks(cave_map: &mut HashMap<Point, char>, line: String) -> (usize, usize) {
    let mut prev_point = Point {x: 0, y: 0};
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    
    for token in line.split(" -> ") {
	let point = Point {
	    x: token.split(",").nth(0).unwrap().parse::<usize>().expect("Failed to parse x"),
	    y: token.split(",").nth(1).unwrap().parse::<usize>().expect("Failed to parse y"),
	};

	if point.x > max_x {
	    max_x = point.x;
	}
	if point.y > max_y {
	    max_y = point.y;
	}
	
	if prev_point.x != 0 || prev_point.y != 0 {
	    if point.x < prev_point.x {
		for idx in (point.x..=prev_point.x).rev() {
		    prev_point.x = idx;
		    cave_map.insert(prev_point.clone(), '#');
		}
	    }
	    else if point.x > prev_point.x {
		for idx in prev_point.x..=point.x {
		    prev_point.x = idx;
		    cave_map.insert(prev_point.clone(), '#');
		}
	    }

	    if point.y < prev_point.y {
		for idx in (point.y..=prev_point.y).rev() {
		    prev_point.y = idx;
		    cave_map.insert(prev_point.clone(), '#');
		}
	    }
	    else if point.y > prev_point.y {
		for idx in prev_point.y..=point.y {
		    prev_point.y = idx;
		    cave_map.insert(prev_point.clone(), '#');
		}
	    }
	}
	else {
	    prev_point.x = point.x;
	    prev_point.y = point.y;
	}
    }

    return (max_x, max_y);
}
