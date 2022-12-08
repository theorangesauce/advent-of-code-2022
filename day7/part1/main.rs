use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::vec::Vec;

struct Filesystem {
    current_directory: String,
    dir_child_map: HashMap<String, HashSet<String>>,
    dir_parent_map: HashMap<String, String>,
    file_size_map: HashMap<String, u64>,
    directory_set: BTreeSet<String>,
}

impl Filesystem {
    // Adds file or dir child_name as a child of the current directory
    // If child_size is None, new child is a directory
    fn add_child(&mut self, child_name: &str, child_size: Option<u64>) {
	let full_child_path = self.current_directory.clone() + "/" + child_name;

	self.dir_child_map.get_mut(&self.current_directory)
	    .expect("Current directory not visited?")
	    .insert(full_child_path.to_string().clone());
	
	match child_size {
	    Some(size) => {
		self.file_size_map.insert(full_child_path.to_string(), size);
	    },
	    None => {
		if !self.dir_child_map.contains_key(&full_child_path.to_string()) {
		    self.dir_child_map.insert(full_child_path.to_string(), HashSet::new());
		}
		self.dir_parent_map.insert(full_child_path.to_string(), self.current_directory.clone());
		self.directory_set.insert(full_child_path.to_string());
	    },
	}

	return;
    }

    // Sets current directory to the root directory ("/")
    // Returns new current directory.
    fn move_to_root(&mut self) -> String {
	if self.dir_child_map.len() == 0 {
	    self.dir_child_map.insert("".to_string(), HashSet::new());
	    self.directory_set.insert("".to_string());
	}

	self.current_directory = "".to_string();

	return self.current_directory.clone();
    }
    
    // Change current directory to that directory's parent.
    // Returns new current dir if successful, None otherwise
    fn move_to_parent_dir(&mut self) -> Option<String> {
	match self.dir_parent_map.get(&self.current_directory) {
	    Some(parent) => {
		self.current_directory = parent.to_string();
		return Some(parent.to_string());
	    },
	    None => return None,
	}
    }

    // Change current directory to child.
    // Returns new current dir if successful, None otherwise 
    fn move_to_child_dir(&mut self, child: &str) -> Option<String> {
	let full_child_path = self.current_directory.clone() + "/" + child;
	
	if self.dir_child_map.get_mut(&self.current_directory)
	    .expect("{self.current_directory} not visited before?")
	    .contains(&full_child_path.to_string()) {
		self.current_directory = full_child_path;
		return Some(self.current_directory.clone());		
	    }
	else {
		return None;
	}
    }

    fn get_dir_size(&mut self, dir: &String) -> u64 {
	let mut size: u64 = 0;

	let children = self.dir_child_map.get(dir).expect("Directory does not exist!").clone();
	for child in children {
	    match self.file_size_map.get(&child.clone()) {
		Some(s) => {
		    size += s;
		},
		None => {
		    size += self.get_dir_size(&child.clone())
		},
	    }
	}

	self.file_size_map.insert(dir.to_string(), size);

	return size;
    }

    fn _print_directory_map(&self, start_dir: String, start_level: usize) {
	for _ in 0..start_level {
	    print!("|");
	}
	println!("= {start_dir}");
	
	for item in self.dir_child_map.get(&start_dir).expect("Directory does not exist!") {
	    if self.directory_set.contains(&item.clone()) {
		self._print_directory_map(item.clone(), start_level + 1);
	    }
	    else {
		for _ in 0..(start_level+1) {
		    print!("|");
		}
		println!("= {} [{} kb]", item.clone(), self.file_size_map.get(&item.clone()).unwrap());
	    }
	}
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

    let commands = split_commands(&input);

    let mut state = Filesystem {
	current_directory: "".to_string(),
	dir_child_map: HashMap::new(),
	dir_parent_map: HashMap::new(),
	file_size_map: HashMap::new(),
	directory_set: BTreeSet::new(),
    };

    for command in commands {
	update_state(&command, &mut state);
    }

    let directory_sizes = get_directory_sizes(&mut state);
    let mut total_size = 0;

    for size in directory_sizes {
	if size > 100000 {
	    break;
	}

	total_size += size;
    }

    println!("The total size of all directories below 100,000 is {total_size}"); 
    
}

fn get_directory_sizes(state: &mut Filesystem) -> Vec<u64> {
    let mut largest_directories: Vec<u64> = Vec::new();
    for dir in &state.directory_set.clone() {
	let dir_size: u64 = state.get_dir_size(&dir);
	largest_directories.push(dir_size);
    }

    largest_directories.sort_unstable();
    return largest_directories;
}

fn update_state(command: &Vec<String>, state: &mut Filesystem) {
    for line in command {
	if line.starts_with("$") {
	    match line.split(' ').nth(2) {
		Some(dest_dir) => {
		    match dest_dir {
			"/" => {
			    state.move_to_root();
			},
			".." => {
			    state.move_to_parent_dir()
				.expect("Directory {state.current_directory} has no known parent!");
			},
			_ => {
			    state.move_to_child_dir(dest_dir)
				.expect("Failed to move to child directory!");
			},
		    }
		},
		None => continue,
	    }
	}
	else {
	    let mut tokens = line.split(' ');
	    let item_type = tokens.nth(0).expect("Line empty?");
	    let item_name = tokens.nth(0).expect("Line empty?");

	    match item_type {
		"dir" => state.add_child(&item_name, None),
		_ => state.add_child(&item_name, Some(item_type.parse::<u64>().expect("Invalid file size"))),
	    }

	}
    }
}

fn split_commands(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut commands: Vec<Vec<String>> = Vec::new();
    commands.push(Vec::new());
    
    let mut iter = input.iter().peekable();
    while let Some(line) = iter.next() {
	commands.last_mut().unwrap().push(line.to_string());
	match iter.peek() {
	    Some(next_line) => {
		match next_line.chars().nth(0) {
		    Some('$') => {
			commands.push(Vec::new());
			
		    },
		    _ => continue
		}
	    },
	    None => continue
	}
    }

    return commands;
}

fn _print_all_commands(commands: Vec<Vec<String>>) {
    for command in commands {
	for line in command {
	    println!("{line}");
	}
	println!("--------");
    }
}
