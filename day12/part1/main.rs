use std::io;
use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point{
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

    let (elevation_map, start_point, dest_point) = get_elevation_map(&input);

    let mut distance_scores: HashMap<Point, u32> = HashMap::new();
    let mut point_queue: VecDeque<(Point, u32)> = VecDeque::new();
    point_queue.push_back((start_point.clone(), 0));
    
    while !point_queue.is_empty() && !distance_scores.contains_key(&dest_point) {
	let (point, score) = point_queue.pop_front().unwrap();
	match distance_scores.get(&point) {
	    Some(n) => {
		if &score < n {
		    distance_scores.insert(point.clone(), score);
		}
		else {
		    continue;
		}
	    },
	    None => {
		distance_scores.insert(point.clone(), score);
	    },
	}
	let new_point_scores = get_scores(&elevation_map, &point, score);

	for (new_point, new_score) in new_point_scores {
	    match distance_scores.get(&new_point) {
		Some(n) => {
		    if &new_score < n {
			point_queue.push_back((new_point.clone(), new_score));
		    }
		},
		None => point_queue.push_back((new_point.clone(), new_score)),
	    }
	}
    }

    println!("The best route to the endpoint takes {} steps", distance_scores.get(&dest_point).unwrap());
}

fn get_scores(elevation_map: &HashMap<Point, char>, point: &Point, score: u32) -> Vec<(Point, u32)> {
    let mut new_point_scores: Vec<(Point, u32)> = Vec::new();
    let elevation = elevation_map.get(point).unwrap().clone();

    let up_point = Point{x: point.x, y: point.y + 1};
    match elevation_map.get(&up_point) {
	Some(n) => {
	    if elevation as u32 + 1 >= *n as u32 {
		new_point_scores.push((up_point, score + 1));
	    }
	},
	None => {},
    }

    if point.y > 0 {
	let down_point = Point{x: point.x, y: point.y - 1};
	match elevation_map.get(&down_point) {
	    Some(n) => {
		if elevation as u32 + 1 >= *n as u32 {
		    new_point_scores.push((down_point, score + 1));
		}
	    },
	    None => {},
	}
    }
    
    if point.x > 0 {
	let left_point = Point{x: point.x - 1, y: point.y};
	match elevation_map.get(&left_point) {
	    Some(n) => {
		if elevation as u32 + 1 >= *n as u32 {
		    new_point_scores.push((left_point, score + 1));
		}
	    },
	    None => {},
	}
    }

    let right_point = Point{x: point.x + 1, y: point.y};
    match elevation_map.get(&right_point) {
	Some(n) => {
	    if elevation as u32 + 1 >= *n as u32 {
		new_point_scores.push((right_point, score + 1));
	    }
	},
	None => {},
    }

    return new_point_scores;
}

fn get_elevation_map(input: &Vec<String>) -> (HashMap<Point, char>, Point, Point) {
    let mut elevation_map: HashMap<Point, char> = HashMap::new();
    let mut point = Point {x: 0, y: 0};
    let mut start_point = point.clone();
    let mut dest_point = point.clone();

    
    for line in input {
	for position in line.chars() {
	    if position == 'E' {
		elevation_map.insert(point.clone(), 'z');
		dest_point = point.clone();
	    }
	    else if position == 'S' {
		elevation_map.insert(point.clone(), 'a');
		start_point = point.clone();
	    }
	    else {
		elevation_map.insert(point.clone(), position);
	    }
	    point.x += 1;
	}
	point.x = 0;
	point.y += 1;
    }
    return (elevation_map, start_point, dest_point);
}
