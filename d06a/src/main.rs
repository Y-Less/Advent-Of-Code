use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::LinkedList;

fn recurse(system: &HashMap<&str, LinkedList<&str>>, planet: &str, depth: u32) -> u32
{
	match system.get(planet)
	{
		Some(orbits) => orbits.iter().fold(depth, |acc, x| acc + recurse(system, x, depth + 1)),
		None => depth,
	}
}

fn main() -> std::io::Result<()>
{
	let mut contents = String::new();
	{
		let mut file = File::open("../inputs/d06.txt")?;
		file.read_to_string(&mut contents)?;
	}
	let contents = contents;
//    assert_eq!(contents, "Hello, world!");
//}
	//let mut prog = prog.trim().split(',')
	//	.map(|x| x.parse().expect("Not a number"))

	let mut system: HashMap<&str, LinkedList<&str>> = HashMap::new();

	contents.trim().split('\n')
		.for_each(|x|
		{
			let mut ab = x.split(')');//.collect();
			
			let centre = ab.next().unwrap_or("___");
			let planet = ab.next().unwrap_or("___");
		
			let opt = system.get_mut(centre).take();
			let mut orbits: LinkedList<&str> = LinkedList::new();
			match opt
			{
				Some(l2) =>
				{
					l2.iter().for_each(|x| orbits.push_back(x));
					//let mut list: LinkedList<&str> = LinkedList::new();
					//list.push_back(planet);
					//system.insert(centre, list);
				}
				None =>
				{
					//let mut list: LinkedList<&str> = LinkedList::new();
					//list.push_back(planet);
				}
			}
			// Need to copy the list for some reason...  I'm sure there's a better way.
			orbits.push_back(planet);
			system.insert(centre, orbits);

		});
	
	//system.iter().for_each(|x| println!("{:?} ", x));
	
	//println!("{} is orbited by {}", centre, planet);
	
	println!("{}", recurse(&system, "COM", 0));
	
	Ok(())
}
