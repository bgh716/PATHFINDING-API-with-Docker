use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct UI {
	map: Vec<Vec<i32>>,
	starts: Vec<Vec<i32>>,
	goals: Vec<Vec<i32>>,
	agents: i32,
	pub hValues: Vec<Vec<Vec<i32>>>,
}

impl UI {
	pub fn new(map: Vec<Vec<i32>>, starts: Vec<Vec<i32>>, goals: Vec<Vec<i32>>, agents: i32) -> UI {
		let mut hValues = Vec::new();
		for i in 0..agents{
			hValues.push(calc_hValues(map.clone(), goals[i as usize].clone()))
		}
		UI{
			map,
			starts,
			goals,
			agents,
			hValues,
		}
	}
	
	pub fn find_solution(&self) -> Option<Vec<Vec<Vec<i32>>>>{
		let mut paths = Vec::new();
		for i in 0..self.agents {
			let mut path = a_star(self.map.clone(),
								self.starts[i as usize].clone(),
								self.goals[i as usize].clone(),
								self.hValues[i as usize].clone());
			if (path != None){
				paths.push(path.unwrap().clone());
			}
			else{
				return None;
			}
		}
		Some(paths)
	}
}

pub fn a_star(map: Vec<Vec<i32>>, start: Vec<i32>, goal: Vec<i32>, hValues: Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>>{
	let mut open_list = BinaryHeap::new();
	let mut closed_list = Vec::new();
	let mut hValueI = hValues[start[0] as usize][start[1] as usize];
	let root = Child::new(start, 0, hValueI, None);
	open_list.push(root.clone());
	closed_list.push(root);
	while open_list.len() > 0 {
		let mut curr = open_list.pop().unwrap();
		if (curr.loc == goal){
			return Some(get_path(curr));
		}
		for i in 0..4{
			let mut new_loc = movement(&curr.loc, i);
			if (new_loc[0] < 0 || new_loc[0] >= map.len() as i32 || new_loc[1] < 0 || new_loc[1] >= map[0].len() as i32){
				continue;
			}
			if (map[new_loc[0] as usize][new_loc[1] as usize] == 1){
				continue;
			}
			let mut new_child = Child::new(new_loc.clone(), curr.g_val + 1, hValues[new_loc[0] as usize][new_loc[1] as usize], Some(Box::new(curr.clone())));
			let mut found = 0;
			for existing_child in closed_list.iter_mut(){
				if (new_child.loc == existing_child.loc ){
					found = 1;
					if (new_child.f_val < existing_child.f_val){
						*existing_child = new_child.clone();
						open_list.push(new_child.clone());
						break;
					}
					else{
						break;
					}
				}
			}
			if (found == 0){
				closed_list.push(new_child.clone());
				open_list.push(new_child);
			}
		}
	}
	None
}

pub fn get_path(node: Child) -> Vec<Vec<i32>> {
	let mut path = Vec::new();
	let mut curr = node.clone();
	path.push(node.loc.clone());
	while curr.parent != None {
		curr = *curr.parent.clone().unwrap();
		path.push(curr.loc.clone());
	}
	path
}	

#[derive(Debug, Clone, Eq)]
pub struct Child {
	loc: Vec<i32>,
	g_val: i32,
	h_val: i32,
	f_val: i32,
	parent: Option<Box<Child>>,
}

impl Child {
	pub fn new(loc: Vec<i32>, g_val: i32, h_val: i32, parent: Option<Box<Child>>) -> Child {
		let f_val = g_val + h_val;
		Child{
			loc,
			g_val,
			h_val,
			f_val,
			parent
		}
	}
}

impl Ord for Child {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.f_val.cmp(&other.f_val) != Ordering::Equal){
			if (self.f_val.cmp(&other.f_val) == Ordering::Less){
				Ordering::Greater
			}
			else{
				Ordering::Less
			}
		}
        else if (self.h_val.cmp(&other.h_val) != Ordering::Equal){
			if (self.h_val.cmp(&other.h_val) == Ordering::Less){
				Ordering::Greater
			}
			else{
				Ordering::Less
			}
		}
		else{
			if (self.loc.cmp(&other.loc) == Ordering::Equal){
				Ordering::Equal
			}
			else if (self.loc.cmp(&other.loc) == Ordering::Less){
				Ordering::Greater
			}
			else{
				Ordering::Less
			}
		}
    }
}

impl PartialOrd for Child {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Child {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.g_val == other.g_val && self.h_val == other.h_val && self.f_val == other.f_val
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Child_H {
	loc: Vec<i32>,
	cost: i32,
}

impl Child_H {
	pub fn new(loc: Vec<i32>, cost: i32) -> Child_H {
		Child_H{
			loc,
			cost
		}
	}
}

impl Ord for Child_H {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.cost.cmp(&other.cost) != Ordering::Equal){
			if (self.cost.cmp(&other.cost) == Ordering::Less){
				Ordering::Greater
			}
			else{
				Ordering::Less
			}
		}
		else{
			if (self.loc.cmp(&other.loc) == Ordering::Equal){
				Ordering::Equal
			}
			else if (self.loc.cmp(&other.loc) == Ordering::Less){
				Ordering::Greater
			}
			else{
				Ordering::Less
			}
		}
    }
}

impl PartialOrd for Child_H {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Child_H {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.cost == other.cost
    }
}

pub fn movement(loc: &Vec<i32>, dir: i32) -> Vec<i32> {
	let mut new_loc = loc.clone();
	if (dir == 0) {
		new_loc[1] = new_loc[1] - 1;
	}
	else if (dir == 1) {
		new_loc[0] = new_loc[0] - 1;
	}
	else if (dir == 2) {
		new_loc[1] = new_loc[1] + 1;
	}
	else if (dir == 3) {
		new_loc[0] = new_loc[0] + 1;
	}
	
	new_loc
}

pub fn calc_hValues(map: Vec<Vec<i32>>, goal: Vec<i32>) -> Vec<Vec<i32>> {
	let mut open_list = BinaryHeap::new();
	let mut closed_list = Vec::new();
	let root = Child_H::new(goal, 0);
	closed_list.push(root.clone());
	open_list.push(root);
	while open_list.len() > 0 {
		let mut curr = open_list.pop().unwrap();
		for i in 0..4{
			let mut new_loc = movement(&curr.loc, i);
			let mut new_cost = curr.cost + 1;
			if (new_loc[0] < 0 || new_loc[0] >= map.len() as i32 || new_loc[1] < 0 || new_loc[1] >= map[0].len() as i32){
				continue;
			}
			if (map[new_loc[0] as usize][new_loc[1] as usize] == 1){
				continue;
			}
			let mut new_child = Child_H::new(new_loc,new_cost);
			let mut found = 0;
			for existing_child in closed_list.iter_mut(){
				if (new_child.loc == existing_child.loc ){
					found = 1;
					if (new_child.cost < existing_child.cost){
						//println!("{:?}", new_child);
						//println!("{:?}", existing_child);
						*existing_child = new_child.clone();
						open_list.push(new_child.clone());
						break;
					}
					else{
						break;
					}
				}
			}
			if (found == 0){
				//println!("{:?}", new_child);
				closed_list.push(new_child.clone());
				open_list.push(new_child);
			}
		}
	}
	let mut hValues = map.clone();
	for child in closed_list.iter(){
		hValues[child.loc[0] as usize][child.loc[1] as usize] = child.cost;
	}
	
	hValues
}