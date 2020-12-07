use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;
use regex::*;

type Bag = (String, i64);
type Contains = LinkedList<Bag>;
type Nesting = HashMap<String, Contains>;

fn get_child(name: Option<Match>, count: Option<Match>) -> Bag
{
	let count = count.unwrap().as_str();
	let name = name.unwrap().as_str();
	let count = count.parse().expect("Bag amount not a number");

	(name.to_string(), count)
}

fn get_parents(children: &Nesting, name: &str) -> Contains
{
	let mut parents = LinkedList::new();
	for (key, value) in children
	{
		for (n, c) in value
		{
			if n == name
			{
				parents.push_back((key.to_string(), *c));
			}
		}
	}

	parents
}

fn get_ancestors(ancestors: &mut HashSet<String>, parents: &Nesting, name: &str)
{
	for (k, _v) in parents.get(name).unwrap()
	{
		ancestors.insert(k.to_string());
		get_ancestors(ancestors, parents, k);
	}
}

fn get_children(captures: &Captures) -> Contains
{
	// Guide to (useful) matches:
	//
	//   1 - parent
	//   3 - no other bags
	//   5 - count 1
	//   6 - child 1
	//   8 - count 2
	//   9 - child 2
	//   11 - count 3
	//   12 - child 3
	//   14 - count 4
	//   15 - child 4
	//
	let mut children = LinkedList::new();
	match captures.get(5)
	{
		None => {}
		Some(_x) =>
		{
			children.push_back(get_child(captures.get(6), captures.get(5)));
		}
	}
	match captures.get(8)
	{
		None => {}
		Some(_x) =>
		{
			children.push_back(get_child(captures.get(9), captures.get(8)));
		}
	}
	match captures.get(11)
	{
		None => {}
		Some(_x) =>
		{
			children.push_back(get_child(captures.get(12), captures.get(11)));
		}
	}
	match captures.get(14)
	{
		None => {}
		Some(_x) =>
		{
			children.push_back(get_child(captures.get(15), captures.get(14)));
		}
	}

	children
}

fn main()
{
	let mut children: Nesting = HashMap::new();
	let mut parents: Nesting = HashMap::new();
	let mut input = String::new();
	let mut file = File::open("../inputs/d07.txt").expect("Could not read bags file");
	file.read_to_string(&mut input).expect("Could not read bags file");

	// Very cheating...
	let re = Regex::new(r"(\w+ \w+) bags contain ((no other bags)|((\d+) (\w+ \w+) bags?)(, (\d+) (\w+ \w+) bags?)?(, (\d+) (\w+ \w+) bags?)?(, (\d+) (\w+ \w+) bags?)?)\.").unwrap();

	// Guide to (useful) matches:
	//
	//   1 - parent
	//   3 - no other bags
	//   5 - count 1
	//   6 - child 1
	//   8 - count 2
	//   9 - child 2
	//   11 - count 3
	//   12 - child 3
	//   14 - count 4
	//   15 - child 4
	//
	input.trim().split('\n')
		.for_each(|x|
		{
			let cap = re.captures(x).unwrap();
			children.insert(cap.get(1).unwrap().as_str().to_string(), get_children(&cap));
		});

	for (name, _v) in children.iter()
	{
		parents.insert(name.to_string(), get_parents(&children, &name));
	}

	println!("{:?}", parents);

	let mut ancestors: HashSet<String> = HashSet::new();
	get_ancestors(&mut ancestors, &parents, "shiny gold");

	println!("{:?}", ancestors);
	println!("{:?}", ancestors.len());
}

