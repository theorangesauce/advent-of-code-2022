use std::io;
use std::vec::Vec;
use std::collections::BTreeSet;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

struct Sensor {
    center: Point,
    beacon: Point,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    // Checks if rhs intersects with self on self's end side
    // Returns true if ranges combined, false otherwise
    fn try_combine_right(&mut self, rhs: &Range) -> bool {
	if self.start <= rhs.start && rhs.start <= self.end + 1 {
	    if self.end < rhs.end {
		self.end = rhs.end;
	    }
	    return true;
	}
	else {
	    return false;
	}
    }

    // Returns the number of points covered, inclusive
    fn _points_covered(&self) -> i32 {
	return self.end - self.start + 1;
    }
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

    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: BTreeSet<Point> = BTreeSet::new();
    
    for line in input {
	sensors.push(create_sensor(line));
    }

    for sensor in &sensors {
	beacons.insert(Point {x: sensor.beacon.x, y: sensor.beacon.y});
    }

    let mut candidate_points: BTreeSet<Point> = BTreeSet::new();
    const MIN_VAL: i32 = 0;
    const MAX_VAL: i32 = 4000000;
    
    for y_to_check in MIN_VAL..MAX_VAL {
	let mut ranges: BTreeSet<Range> = BTreeSet::new();
	for sensor in &sensors {
	    match get_range_covered(&sensor, y_to_check) {
		Some(n) => ranges.insert(n),
		None => continue,
	    };
	}

	merge_overlapping_ranges(&mut ranges);
	constrain_x_values(&mut ranges, MIN_VAL, MAX_VAL);
	candidate_points.append(&mut get_beacon_candidates(&ranges, y_to_check, MIN_VAL, MAX_VAL));
    }

    let possible_points = candidate_points.difference(&beacons).collect::<Vec<&Point>>();
    
    if possible_points.len() == 1 {
	let point = possible_points.first().unwrap();
	let val: i64 = (point.x as i64) * 4000000 + (point.y as i64);

	println!("The tuning frequency of the missing beacon is {val}");
    }
    else {
	println!("No single candidate for beacon found!");

	print!("Possible candidates: ");
	for point in possible_points {
	    print!("({}, {}), ", point.x, point.y);
	}
	println!();
    }
}

fn get_beacon_candidates(ranges: &BTreeSet<Range>, y_val: i32, min_x: i32, max_x: i32) -> BTreeSet<Point> {
    let mut candidate_points: BTreeSet<Point> = BTreeSet::new();
    let default_range = Range {start: min_x, end: max_x};
    
    let ranges_first = ranges.iter().next().unwrap_or(&default_range);
    for x_val in min_x..(ranges_first.start) {
	candidate_points.insert(Point {x: x_val, y: y_val});
    }

    let ranges_last = ranges.iter().next_back().unwrap_or(&default_range);
    for x_val in (ranges_last.end + 1)..(max_x + 1) {
	candidate_points.insert(Point {x: x_val, y: y_val});
    }

    let mut left_iter = ranges.iter();
    let mut right_iter = ranges.iter();
    right_iter.next();
    
    while right_iter.len() > 0 {
	let ranges_left = left_iter.next().unwrap();
	let ranges_right = right_iter.next().unwrap();

	for x_val in (ranges_left.end + 1)..ranges_right.start {
	    candidate_points.insert(Point {x: x_val, y: y_val});
	}
    }

    return candidate_points;
}

fn constrain_x_values(ranges: &mut BTreeSet<Range>, min_x: i32, max_x:i32 ) {
    let mut constrained_ranges: BTreeSet<Range> = BTreeSet::new();
    
    for range in ranges.iter() {
	let mut new_range = range.clone();
	if new_range.start < min_x {
	    new_range.start = min_x;
	}
	
	if new_range.end > max_x {
	    new_range.end = max_x;
	}

	if new_range.start <= new_range.end {
	    constrained_ranges.insert(new_range);
	}
    }

    *ranges = constrained_ranges;
}

fn merge_overlapping_ranges(ranges: &mut BTreeSet<Range>) {
    let mut merged_ranges: BTreeSet<Range> = BTreeSet::new();
    let mut removed_ranges: BTreeSet<Range> = BTreeSet::new();
    
    for range in ranges.iter() {
	let mut new_range = range.clone();
	
	if removed_ranges.contains(&new_range) {
	    continue;
	}
	else {
	    let larger_ranges = ranges.clone().split_off(&range);
	    
	    for larger_range in larger_ranges {
		
		if &new_range == &larger_range {
		    continue;
		}
		else if new_range.try_combine_right(&larger_range) {
		    removed_ranges.insert(larger_range.clone());
		}
		else {
		    break;
		}
	    }
	}

	merged_ranges.insert(new_range);
    }

    *ranges = merged_ranges;
}

fn get_range_covered(sensor: &Sensor, y_val: i32) -> Option<Range> {
    let manhattan_distance = (sensor.beacon.x - sensor.center.x).abs() + (sensor.beacon.y - sensor.center.y).abs();

    let y_distance = (y_val - sensor.center.y).abs();
    if y_distance > manhattan_distance {
	return None
    }
    else {
	let mut start = sensor.center.x - (manhattan_distance - y_distance);
	let mut end = sensor.center.x + (manhattan_distance - y_distance);

	if start == sensor.beacon.x && y_val == sensor.beacon.y {
	    start += 1;
	}
	if end == sensor.beacon.x  && y_val == sensor.beacon.y {
	    end -= 1;
	}

	if start > end {
	    return None;
	}

	return Some(Range {start: start, end: end});
    }
}

fn create_sensor(line: String) -> Sensor {
    let mut left_pair = line.split(": ").nth(0).unwrap()
	.strip_prefix("Sensor at x=").expect("Malformed sensor").split(", y=");
    let mut right_pair = line.split(": ").nth(1).unwrap()
	.strip_prefix("closest beacon is at x=").expect("Malformed beacon").split(", y=");

    let center_x = left_pair.nth(0).unwrap();
    let center_y = left_pair.nth(0).unwrap();
    let beacon_x = right_pair.nth(0).unwrap();
    let beacon_y = right_pair.nth(0).unwrap();
    
    Sensor {
	center: Point {
	    x: center_x.parse::<i32>().expect("Invalid center x value"),
	    y: center_y.parse::<i32>().expect("Invalid center y value"),
	},
	beacon: Point {
	    x: beacon_x.parse::<i32>().expect("Invalid center x value"),
	    y: beacon_y.parse::<i32>().expect("Invalid center y value"),
	}
    }
}
