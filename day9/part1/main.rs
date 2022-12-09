use std::io;
use std::vec::Vec;
use std::collections::HashSet;

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
	tail.1 = head.1;
    }
    else if (head.0 - tail.0) < -1 {
	tail.0 -= 1;
	tail.1 = head.1;
    }
    
    // y
    if (head.1 - tail.1) > 1 {
	tail.1 += 1;
	tail.0 = head.0;
    }
    else if (head.1 - tail.1) < -1 {
	tail.1 -= 1;
	tail.0 = head.0;
    }
}

fn get_visited_point_count(input: &Vec<String>) -> usize {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);

    for line in input {
	let distance: i32 = line.split(' ').nth(1).unwrap()
	    .parse::<i32>().expect("Invalid distance");

	match line.chars().nth(0).unwrap() {
	    'U' => {
		for _ in 0..distance {
		    head.1 += 1;
		    move_tail(&head, &mut tail);
		    visited_points.insert(tail);
		}
	    },
	    'D' => {
		for _ in 0..distance {
		    head.1 -= 1;
		    move_tail(&head, &mut tail);
		    visited_points.insert(tail);
		}
	    },
	    'L' => {
		for _ in 0..distance {
		    head.0 -= 1;
		    move_tail(&head, &mut tail);
		    visited_points.insert(tail);
		}
	    },
	    'R' => {
		for _ in 0..distance {
		    head.0 += 1;
		    move_tail(&head, &mut tail);
		    visited_points.insert(tail);
		}
	    },
	    _ => println!("Invalid instruction"),
	}
    }
    
    return visited_points.len();
}
