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
	let mut children = LinkedList::new();

	input.split(", ")
		.for_each(|x| children.push_back(get_child(x)));

	children
}

//fn get_parents_of(children: &Nesting, name: &str) -> LinkedList<Bag>
//{
//	let mut parents: LinkedList<Bag> = LinkedList::new();
//	for (key, value) in children
//	{
//		for child in value
//		{
//			if name == child._0
//			{
//				parents.push_back((key, child._1))
//			}
//		}
//	}
//	parents
//}
//
//fn get_all_parents(children: &Nesting) -> Nesting
//{
//	let mut parents: Nesting = HashMap::new();
//	for (key, value) in children
//	{
//		let mut parent = LinkedList::new();
//		for child in value
//		{
//			parent.push_back((key, child._1))
//		}
//	}
//	parents
//}

//// Excess = (amount made at once, excess, ore required to make all at once)
//fn get_ore_count(reactions: &Reactions, excess: &mut HashMap<String, (i64, i64, i64)>, chemical: String, num: i64, taken: &mut i64) -> i64
//{
//	if chemical == "ORE"
//	{
//		// Need `n` ORE to make `n` ORE.
//		*taken = *taken + num;
//		return num;
//	}
//
//	let mut num = num;
//	match excess.get(&chemical)
//	{
//	Some(n) =>
//	{
//		if n.1 >= num
//		{
//			let per_loop = n.0;
//			let num = n.1 - num;
//			let req = n.2;
//			excess.insert(chemical.to_string(), (per_loop, num, req));
//			return 0;
//		}
//		// Already made some of this, so no more raw inputs required.
//		num = num - n.1;
//		// We may end up putting some back in later.
//		excess.insert(chemical.to_string(), (0, 0, 0));
//	}
//	None => {}
//	}
//
//	match reactions.get(&chemical)
//	{
//	Some(input) =>
//	{
//		let per_loop = input.0;
//		let reactants = &input.1;
//		let mut ret = 0;
//		loop
//		{
//			// Get the ORE required to make one set of this input.
//			let req = reactants.iter().fold(0, |a, x| a + get_ore_count(reactions, excess, x.0.to_string(), x.1, taken));
//			ret += req;
//			// One call to the reduction above gives enough inputs to make `per_loop` reactants.
//			if per_loop >= num
//			{
//				// We may have excess.
//				excess.insert(chemical.to_string(), (per_loop, per_loop - num, req));
//				break;
//			}
//			num = num - per_loop;
//		}
//		ret
//	}
//	None => panic!("Can't find chemical inputs."),
//	}
//}

fn main()
{
	let mut children: Nesting = HashMap::new();
	let mut input = String::new();
	let mut file = File::open("../inputs/d07.txt").expect("Could not read bags file");
	file.read_to_string(&mut input).expect("Could not read bags file");

	// Very cheating...
	let re = Regex::new(r"(\w+) (\w+) bags contain ((no other bags)|((\d+) (\w+) (\w+) bags?)(, (\d+) (\w+) (\w+) bags?)?(, (\d+) (\w+) (\w+) bags?)?(, (\d+) (\w+) (\w+) bags?)?)\.").unwrap();

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
				None => {}
				}
			}
			//println!("{} {} {}", cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str());

			//let i = io.next().unwrap();
			//let o = io.next().unwrap();
            //
			//let c = get_chemical(o);
			//children.insert(c.0, (c.1, get_children(i)));
		});

//	let mut excess = HashMap::new();
//	// 301997 for one.
//	let mut taken = 0;
//	//const UNITS: i64 = 1000000;
//	let mut max = 0;
//	const UPPER: i32 = 100000000;
//	for UNITS in 1 .. UPPER
//	{
//		if UNITS % (UPPER / 100) == 0
//		{
//			println!("{}%", UNITS * 100 / UPPER);
//		}
//		get_ore_count(&bags, &mut excess, "FUEL".to_string(), 1, &mut taken);
//		//println!("{:?}", excess);
//
//		// Add up all the excess ORE used in the production process.
//		
//		
//		//const FUEL: f64 = 301997.0;
//		let mut spare: f64 = 0.0;
//		for i in excess.iter()
//		{
//			spare += ((i.1).1 as f64 / (i.1).0 as f64) * (i.1).2 as f64;
//		}
//		//println!("{}", taken);
//		//println!("{:?}", (taken as f64 - spare));
//		//println!("{:?}", 1000000000000.0 / (taken as f64 - spare) * UNITS as f64);
//		//let total = (1000000000000.0 / (taken as f64 - spare) * UNITS as f64) as i64;
//		let total = (1000000000000.0 / (taken as f64) * UNITS as f64) as i64;
//		if total > max
//		{
//			max = total;
//			println!("max = {}", max);
//		}
//	}
	// 6216377.920812947
	// 6216617.746872218
	// 6216579.189618019
	// 6216594.684892516
	// 6216590
	// 6349387
	// 6349388
	// 6216588
}

