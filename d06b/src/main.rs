use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::LinkedList;

fn path_to(system: &HashMap<&str, LinkedList<&str>>, planet: &str, whom: &str) -> Option<LinkedList<String>>
{
	if planet == whom
	{
		println!("Found {} = {}", planet, whom);
		//Some("".to_string())
		
		Some(LinkedList::new())
	}
	else
	{
		match system.get(planet)
		{
			Some(orbits) => orbits.iter()
				.fold(None, |acc, x| acc.or_else(|| path_to(system, x, whom)))
				.and_then(|mut x|
				{
					x.push_front(planet.to_string());
					Some(x)
				}),
				//
				//
				//Some(format!("{}-{}", planet, x))),
			None => None,
		}
	}
}

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
	
	//println!("{}", recurse(&system, "COM", 0));
	//println!("{:?}", path_to(&system, "COM", "SAN").expect("Could not find a path to \"SAN\""));
	//println!("{:?}", path_to(&system, "COM", "YOU").expect("Could not find a path to \"YOU\""));
	
	let p2s = path_to(&system, "COM", "SAN").expect("Could not find a path to \"SAN\"");
	let p2y = path_to(&system, "COM", "YOU").expect("Could not find a path to \"YOU\"");
	
	let mut p2s = p2s.iter();
	let mut p2y = p2y.iter();

	loop
	{
		let a = p2s.next().expect("");
		let b = p2y.next().expect("");
		if a != b
		{
			println!("Found split at {}-{}", a, b);
			let count = p2s.count() + p2y.count() + 2;
			println!("{}", count);
			break;
		}
	}

	Ok(())
}
