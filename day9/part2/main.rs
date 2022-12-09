use std::io;
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32);

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

    let count = get_visited_point_count(&input);

    println!("The tail visited {count} points");
}

fn move_tail(head: &Point, tail: &mut Point) {
    // x
    if (head.0 - tail.0) > 1 {
	tail.0 += 1;
	if head.1 > tail.1 {
	    tail.1 += 1;
	}
	else if head.1 < tail.1 {
	    tail.1 -= 1;
	}
    }
    else if (head.0 - tail.0) < -1 {
	tail.0 -= 1;
	if head.1 > tail.1 {
	    tail.1 += 1;
	}
	else if head.1 < tail.1 {
	    tail.1 -= 1;
	}
    }
    
    // y
    if (head.1 - tail.1) > 1 {
	tail.1 += 1;
	if head.0 > tail.0 {
	    tail.0 += 1;
	}
	else if head.0 < tail.0 {
	    tail.0 -= 1;
	}
    }
    else if (head.1 - tail.1) < -1 {
	tail.1 -= 1;
	if head.0 > tail.0 {
	    tail.0 += 1;
	}
	else if head.0 < tail.0 {
	    tail.0 -= 1;
	}
    }
}

fn get_visited_point_count(input: &Vec<String>) -> usize {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut head = Point(0, 0);
    let mut tails: Vec<Point> = Vec::new();

    for _ in 0..9 {
	tails.push(Point(0, 0));
    }

    for line in input {
	let distance: i32 = line.split(' ').nth(1).unwrap()
	    .parse::<i32>().expect("Invalid distance");

	match line.chars().nth(0).unwrap() {
	    'U' => {
		for _ in 0..distance {
		    head.1 += 1;
		    move_tail(&head, tails.get_mut(0).unwrap());

		    for pos in 1..tails.len() {
			let parent = tails.get(pos - 1).unwrap().clone();
			let child = tails.get_mut(pos).unwrap();

			move_tail(&parent, child);
		    }

		    visited_points.insert(*tails.last().unwrap());
		}
	    },
	    'D' => {
		for _ in 0..distance {
		    head.1 -= 1;
		    move_tail(&head, tails.get_mut(0).unwrap());

		    for pos in 1..tails.len() {
			let parent = tails.get(pos - 1).unwrap().clone();
			let child = tails.get_mut(pos).unwrap();

			move_tail(&parent, child);	
		    }

		    visited_points.insert(*tails.last().unwrap());
		}
	    },
	    'L' => {
		for _ in 0..distance {
		    head.0 -= 1;
		    move_tail(&head, tails.get_mut(0).unwrap());

		    for pos in 1..tails.len() {
			let parent = tails.get(pos - 1).unwrap().clone();
			let child = tails.get_mut(pos).unwrap();

			move_tail(&parent, child);
		    }

		    visited_points.insert(*tails.last().unwrap());
		}
	    },
	    'R' => {
		for _ in 0..distance {
		    head.0 += 1;
		    move_tail(&head, tails.get_mut(0).unwrap());

		    for pos in 1..tails.len() {
			let parent = tails.get(pos - 1).unwrap().clone();
			let child = tails.get_mut(pos).unwrap();

			move_tail(&parent, child);
		    }

		    visited_points.insert(*tails.last().unwrap());
		}
	    },
	    _ => println!("Invalid instruction"),
	}
    }

    return visited_points.len();
}


fn _print_rope(head: &Point, tails: &Vec<Point>) {
    let mut special_points: HashMap<Point, _> = HashMap::new();
    special_points.insert(head.clone(), 'H');
    special_points.insert(Point(0, 0), 'S');
    let mut i: u32 = '1' as u32;
    for tail in tails {
	if special_points.contains_key(tail) {
	    break;
	}
	special_points.insert(tail.clone(), char::from_u32(i).unwrap());
	i += 1;
    }

    for y in (-20..=20).rev() {
	for x in -20..20 {
	    match special_points.get(&Point(x, y)) {
		Some(k) => print!("{k}"),
		None => print!("."),
	    }
	}
	println!();
    }
    println!();
}
