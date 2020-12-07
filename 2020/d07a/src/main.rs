use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

type Bag = (String, i64);
type Nesting = HashMap<String, LinkedList<Bag>>;

fn get_child(input: &str) -> Bag
{
	let mut cn = input.split(' ');

	let count = cn.next().unwrap();
	let name = cn.next().unwrap();
	let count = count.parse().expect("Bag amount not a number");

	(name.to_string(), count)
}

fn get_children(input: &str) -> LinkedList<Bag>
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

	input.split(", ")
		.for_each(|x| children.push_back(get_child(x)));

	children
}

fn main()
{
	let mut children: Nesting = HashMap::new();
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
			//let mut io = x.split(" bags contain ");
			//for cap in re.captures_iter(x)
			//{
			//}
			let caps = re.captures(x).unwrap();
			println!("=========================");
			for cap in caps.iter()
			{
				match cap
				{
				Some(c) =>
				{
					println!("{}", c.as_str());
				}
				None =>
				{
					println!("---");
				}
				}
			}
			//println!("{} {} {}", cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str());

			//let i = io.next().unwrap();
			//let o = io.next().unwrap();
            //
			//let c = get_chemical(o);
			//children.insert(c.0, (c.1, get_children(i)));
		});
}

