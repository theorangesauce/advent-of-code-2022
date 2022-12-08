use std::io;
use std::vec::Vec;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Point(usize, usize);

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

    let map: Vec<Vec<u8>> = generate_map(&input);

    let visible_tree_count: u32 = count_visible_trees(&map);

    println!("{visible_tree_count} trees are visible from the edge");
}

fn count_visible_trees(map: &Vec<Vec<u8>>) -> u32 {
    let col_length = map.len();
    let row_length = map.first().unwrap().len();
    let mut visible_trees: HashSet<Point> = HashSet::new();

    let mut count = (col_length - 1) as u32 * 4;

    // top -> bottom
    for col in 1..(row_length - 1) {
	let mut max_height_seen = map.first().unwrap().get(col).unwrap();
	for row in 1..(col_length - 1) {
	    let tree = map.get(row).unwrap().get(col).unwrap();
	    if tree > max_height_seen {
		max_height_seen = tree;

		let coordinates = Point(row, col);
		if !visible_trees.contains(&coordinates) {
		    visible_trees.insert(coordinates);
		    count += 1;
		}
	    }

	    if *max_height_seen == 9 {
		break;
	    }
	}
    }
    
    // left -> right
    for row in 1..(col_length - 1) {
	let mut max_height_seen = map.get(row).unwrap().first().unwrap();
	for col in 1..(row_length - 1) {
	    let tree = map.get(row).unwrap().get(col).unwrap();
	    if tree > max_height_seen {
		max_height_seen = tree;

		let coordinates = Point(row, col);
		if !visible_trees.contains(&coordinates) {
		    visible_trees.insert(coordinates);
		    count += 1;
		}
	    }

	    if *max_height_seen == 9 {
		break;
	    }
	}
    }

    // right -> left
    for row in 1..(col_length - 1) {
	let mut max_height_seen = map.get(row).unwrap().last().unwrap();
	for col in (1..(row_length - 1)).rev() {
	    let tree = map.get(row).unwrap().get(col).unwrap();
	    if tree > max_height_seen {
		max_height_seen = tree;

		let coordinates = Point(row, col);
		if !visible_trees.contains(&coordinates) {
		    visible_trees.insert(coordinates);
		    count += 1;
		}
	    }

	    if *max_height_seen == 9 {
		break;
	    }
	}
    }
    
    // bottom -> top
    for col in 1..(row_length - 1) {
	let mut max_height_seen = map.last().unwrap().get(col).unwrap();
	for row in (1..(col_length - 1)).rev() {
	    let tree = map.get(row).unwrap().get(col).unwrap();
	    if tree > max_height_seen {
		max_height_seen = tree;

		let coordinates = Point(row, col);
		if !visible_trees.contains(&coordinates) {
		    visible_trees.insert(coordinates);
		    count += 1;
		}
	    }

	    if *max_height_seen == 9 {
		break;
	    }
	}
    }

    return count;
}

fn generate_map(input: &Vec<String>) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = Vec::new();
    
    for line in input {
	map.push(Vec::new());
	let row = map.last_mut().unwrap();
	for tree in line.chars() {
	    row.push(tree as u8 - '0' as u8);
	}
    }

    return map;
}

fn _print_map(map: &Vec<Vec<u8>>) {
    for row in map {
	for item in row {
	    print!("{item}");
	}
	println!("");
    }
}
