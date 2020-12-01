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
	let mut reactants = LinkedList::new();

	input.split(", ")
		.for_each(|x| reactants.push_back(get_chemical(x)));

	reactants
}

type Reactions = HashMap<String, (i32, LinkedList<Chemical>)>;

fn get_ore_count(reactions: &Reactions, excess: &mut HashMap<String, i32>, chemical: String, num: i32) -> i32
{
	if chemical == "ORE"
	{
		// Need `n` ORE to make `n` ORE.
		return num;
	}

	let mut num = num;
	match excess.get(&chemical)
	{
	Some(n) =>
	{
		if *n >= num
		{
			num = *n - num;
			excess.insert(chemical.to_string(), num);
			return 0;
		}
		// Already made some of this, so no more raw inputs required.
		num = num - *n;
		// We may end up putting some back in later.
		excess.insert(chemical.to_string(), 0);
	}
	None => {}
	}

	match reactions.get(&chemical)
	{
	Some(input) =>
	{
		let per_loop = input.0;
		let reactants = &input.1;
		let mut ret = 0;
		loop
		{
			// Get the ORE required to make one set of this input.
			let req = reactants.iter().fold(0, |a, x| a + get_ore_count(reactions, excess, x.0.to_string(), x.1));
			ret += req;
			// One call to the reduction above gives enough inputs to make `per_loop` reactants.
			if per_loop >= num
			{
				// We may have excess.
				excess.insert(chemical.to_string(), per_loop - num);
				break;
			}
			num = num - per_loop;
		}
		ret
	}
	None => panic!("Can't find chemical inputs."),
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

			let c = get_chemical(o);
			reactions.insert(c.0, (c.1, get_reactants(i)));
		});

	let mut excess = HashMap::<String, i32>::new();
	println!("{:?}", get_ore_count(&reactions, &mut excess, "FUEL".to_string(), 1));
}

