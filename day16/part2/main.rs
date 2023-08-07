use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::vec::Vec;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Fingerprint {
    total_pressure: i32,
    time_remaining: i32,
    location: Vec<String>,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    total_pressure: i32,
    time_remaining: i32,
    closed: HashSet<String>,
    open: BTreeSet<String>,
    location: Vec<String>,
}

impl State {
    pub fn new_helper() -> Self {
	State {
	    closed: HashSet::new(),
	    open: BTreeSet::new(),
	    location: Vec::new(),
	    time_remaining: 26,
	    total_pressure: 0,
	}
    }

    pub fn new(closed_valves: Vec<&String>) -> Self {
	let mut state = State::new_helper();
	state.location.push(String::from("AA"));

	for valve in closed_valves {
	    state.closed.insert(valve.clone());
	}

	state
    }

    pub fn create_fingerprint(&self) -> Fingerprint {
	Fingerprint {
	    total_pressure: self.total_pressure,
	    time_remaining: self.time_remaining,
	    location: self.location.clone(),
	}
    }
    
    pub fn open_valve(
	&mut self,
	dest_valve: &String,
	costs: &HashMap<String, i32>,
	distances: &HashMap<(String, String), i32>)
	-> bool
    {
	let path = (self.location.last().unwrap().clone(), dest_valve.clone(),);
	let time_taken = distances.get(&path).unwrap() + 1;

	if time_taken >= self.time_remaining || !self.closed.contains(dest_valve) {
	    return false;
	}

	self.time_remaining -= time_taken;
	self.total_pressure += costs.get(dest_valve).unwrap() * self.time_remaining;
	self.location.push(dest_valve.clone());
	self.closed.remove(dest_valve);
	self.open.insert(dest_valve.clone());

	true
    }

    #[allow(dead_code)]
    pub fn print(&self) {
	println!("LOCATION: {}", self.location.last().unwrap_or(&String::from("?")));
	print!("PATH: ");
	for i in self.location.iter() {
	    print!("{i}, ");
	}
	println!("");
	println!("PRESSURE: {}", self.total_pressure);
	println!("TIME REMAINING: {}", self.time_remaining);
	println!("CLOSED: {}", self.closed.iter().map(|x| String::from(format!("{x}, "))).collect::<String>());
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    pub path: String
}

impl Path {
    pub fn new(valve: &str) -> Self {
	let mut new_path = Path {path: String::from("")};
	new_path.path.push('.');
	new_path.path.push_str(valve);
	new_path
    }
    
    pub fn append(mut self, valve: &str) -> Self {
	self.path.push('.');
	self.path.push_str(valve);
	self
    }

    pub fn len(&self) -> usize {
	self.path.len() / 3
    }

    pub fn endpoint(&self) -> &str {
	&self.path[self.path.len() - 2..]
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

    let mut valves: HashMap<String, Vec<String>> = HashMap::new();
    let mut costs: HashMap<String, i32> = HashMap::new();

    //fill valves & costs
    for line in input {
	let line_split: Vec<&str> = line.split(' ').collect();
	let valve: String = line_split.get(1).unwrap().to_string();
	
	let cost: i32 = line.get(line.find('=').unwrap()+1..line.find(';').unwrap()).unwrap().parse().unwrap();
	if cost > 0 {
	    costs.insert(valve.clone(), cost);
	}
	
	let connections = valves.entry(valve.clone()).or_default();
	
	for token in line_split.iter().rev() {
	    if token.len() <= 3 {
		connections.push(token.get(0..2).unwrap().to_string());
	    }
	    else {
		break;
	    }
	}
    }

    let distances = find_distances(&valves, &costs);

    let (best_path_person, best_path_elephant, max_pressure) = find_max_pressure(&costs, &distances);

    
    println!("The maximum pressure is {max_pressure}.");

    print!("Path (person): ");
    for i in best_path_person {
	print!("{i}, ");
    }
    println!("");

    print!("Path (elephant): ");
    for i in best_path_elephant {
	print!("{i}, ");
    }
    println!("");

}

fn find_distances(
    valves: &HashMap<String, Vec<String>>,
    costs: &HashMap<String, i32>,)
    -> HashMap<(String, String), i32>
{
    let mut distances: HashMap<(String, String), i32> = HashMap::new();

    // distances from initial valve (cost 0)
    let initial_valve = String::from("AA");

    for (dest_valve, _) in costs.iter() {
	let dist = calc_distance(&initial_valve, dest_valve, valves);
	distances.insert((initial_valve.clone(), dest_valve.clone(),), dist);
    }
    
    for (start_valve, _) in costs.clone() {
	for (dest_valve, _) in costs.iter().filter(|&(a, _)| *a != start_valve) {
	    if !distances.contains_key(&(start_valve.clone(), dest_valve.clone(),)) {
		let dist = calc_distance(&start_valve, dest_valve, valves);
		distances.insert((start_valve.clone(), dest_valve.clone(),), dist);
		distances.insert((dest_valve.clone(), start_valve.clone(),), dist);
	    }
	}
    }

    distances
}

fn calc_distance(valve_start: &String, valve_dest: &String, valves: &HashMap<String, Vec<String>>) -> i32 {
    let mut queue: VecDeque<Path> = VecDeque::new();
    queue.push_back(Path::new(valve_start));

    while !queue.is_empty() {
	let current_path = queue.pop_front().unwrap();
	let current_valve = current_path.endpoint();
	let possible_valves: &Vec<String> = valves.get(current_valve).unwrap();

	for next_valve in possible_valves {
	    if next_valve == valve_dest {
		return current_path.len() as i32;
	    }
	    else {
		match current_path.path.find(next_valve) {
		    Some(_) => continue,
		    None => {
			let new_path = current_path.clone().append(next_valve);
			queue.push_back(new_path)
		    },
		}
	    }
	}
    }

    // no path exists between given valves
    panic!();
}

fn find_max_pressure(
    costs: &HashMap<String, i32>,
    distances: &HashMap<(String, String), i32>)
    -> (Vec<String>, Vec<String>, i32)
{
    let mut max_pressure = 0;
    let mut best_path_person: Vec<String> = Vec::new();
    let mut best_path_elephant: Vec<String> = Vec::new();
    let mut stack: Vec<State> = Vec::new();

    let mut seen: HashMap<BTreeSet<String>, Fingerprint> = HashMap::new();
    
    stack.push(State::new(costs.keys().collect::<Vec<&String>>()));

    while !stack.is_empty() {
	let current_state = stack.pop().unwrap();

	match seen.get_mut(&current_state.open) {
	    Some(print) => {
		if current_state.total_pressure > print.total_pressure {
		    print.total_pressure = current_state.total_pressure;
		    print.time_remaining = current_state.time_remaining;
		    print.location = current_state.location.clone();
		}
	    },
	    None => {
		seen.insert(current_state.open.clone(), current_state.create_fingerprint());
	    },
	}
	
	for valve in current_state.closed.iter() {
	    let mut new_state = current_state.clone();
	    if new_state.open_valve(valve, costs, distances) {
		stack.push(new_state);
	    }
	}
    }

    for (opened_person, print_person) in seen.clone().iter() {
	for (_, print_elephant) in seen.iter().filter(|&(a, _)| a.is_disjoint(opened_person)) {
	    if print_elephant.total_pressure + print_person.total_pressure > max_pressure {
		best_path_person = print_person.location.clone();
		best_path_elephant = print_elephant.location.clone();
		max_pressure = print_elephant.total_pressure + print_person.total_pressure;
	    }
	}
    }

    (best_path_person, best_path_elephant, max_pressure,)
}
