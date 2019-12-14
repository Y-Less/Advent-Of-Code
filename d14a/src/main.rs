use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;

type Chemical = (String, i32);

fn get_chemical(input: &str) -> Chemical
{
	let mut cn = input.split(' ');

	let count = cn.next().unwrap();
	let name = cn.next().unwrap();
	let count = count.parse().expect("Chemical amount not a number");

	(name.to_string(), count)
}

fn get_reactants(input: &str) -> LinkedList<Chemical>
{
	let mut ret = LinkedList::new();

	input.split(", ")
		.for_each(|x| ret.push_back(get_chemical(x)));

	ret
}

type Reactions = HashMap<Chemical, LinkedList<Chemical>>;

fn get_ore_count(reactions: &Reactions, cur: String) -> i32
{
	if cur == "ORE"
	{
		return 1;
	}
	match reactions.get(cur)
	{
	Some(l) =>
	{
		l.iter().fold(|a, x| a + x.1 * get_ore_count(x.0), 0)
	}
	None => 0
	}
}

fn main()
{
	let mut reactions: Reactions = HashMap::new();
	let mut input = String::new();
	let mut file = File::open("../inputs/d14.txt").expect("Could not read reactions file");
	file.read_to_string(&mut input).expect("Could not read reactions file");

	input.trim().split('\n')
		.for_each(|x|
		{
			let mut io = x.split(" => ");
	
			let i = io.next().unwrap();
			let o = io.next().unwrap();

			reactions.insert(get_chemical(o), get_reactants(i));
		});
	
	println!("{:?}", get_ore_count(&reactions, "FUEL"));
}

