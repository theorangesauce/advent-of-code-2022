use std::io;
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;

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
    let tree_scenic_scores = get_tree_scenic_scores(&map);

    let mut scores_sorted: Vec<&Point> = tree_scenic_scores.keys().collect::<Vec<&Point>>();
    scores_sorted.sort_unstable_by_key(|key| tree_scenic_scores.get(key).unwrap());

    let best_tree = scores_sorted.last().unwrap();
    println!("The best tree score in the forest is {}, at point ({}, {})",
	     tree_scenic_scores.get(best_tree).unwrap(),
	     best_tree.1, best_tree.0);
}

fn get_tree_scenic_scores(map: &Vec<Vec<u8>>) -> HashMap<Point, u32> {
    let col_length = map.len();
    let row_length = map.first().unwrap().len();
    let mut tree_scenic_scores: HashMap<Point, u32> = HashMap::new();
        
    for col in 1..(row_length - 1) {
	for row in 1..(col_length - 1) {
	    let tree_size = map.get(row).unwrap().get(col).unwrap();
	    let mut count = 1;

	    // up
	    let mut count_up = 0;
	    for row_search in (0..row).rev() {
		count_up += 1;
		if map.get(row_search).unwrap().get(col).unwrap() >= tree_size {
		    break;
		}
	    }
	    
	    count *= count_up;
	    
	    // right
	    let mut count_right = 0;
	    for col_search in (col + 1)..row_length {
		count_right += 1;
		if map.get(row).unwrap().get(col_search).unwrap() >= tree_size {
		    break;
		}
	    }

	    count *= count_right;
	    
	    // down
	    let mut count_down = 0;
	    for row_search in (row + 1)..col_length {
		count_down += 1;
		if map.get(row_search).unwrap().get(col).unwrap() >= tree_size {
		    break;
		}
	    }

	    count *= count_down;
	    
	    // left
	    let mut count_left = 0;
	    for col_search in (0..col).rev() {
		count_left += 1;
		if map.get(row).unwrap().get(col_search).unwrap() >= tree_size {
		    break;
		}
	    }
	    
	    count *= count_left;

	    tree_scenic_scores.insert(Point(row, col), count);
	}
    }
    
    return tree_scenic_scores;    
}

fn _count_visible_trees(map: &Vec<Vec<u8>>) -> u32 {
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
