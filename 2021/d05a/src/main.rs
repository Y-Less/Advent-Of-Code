use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d05.txt")?;
		file.read_to_string(&mut input)?;
	}
	
	let sep = Regex::new(r"\s+->\s+|,").unwrap();
	let mut vents: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
	let mut lines = input.trim().split('\n');
	let mut dangerous = 0;
	for line in lines
	{
		let parts = sep.split(line).map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
		if (parts[0] != parts[2] && parts[1] != parts[3])
		{
			continue;
		}
		//println!("{:?}", line);
		for i in std::cmp::min(parts[0], parts[2])..std::cmp::max(parts[0], parts[2]) + 1
		{
			for j in std::cmp::min(parts[1], parts[3])..std::cmp::max(parts[1], parts[3]) + 1
			{
				//println!("{:?} {:?}", i, j);
				if let Some(x) = vents.get_mut(&i)
				{
					if (x.contains_key(&j))
					{
						if (x[&j] == 1)
						{
							dangerous = dangerous + 1;
						}
						x.insert(j, x[&j] + 1);
					}
					else
					{
						x.insert(j, 1);
					}
				}
				else
				{
					let mut v = HashMap::new();
					v.insert(j, 1);
					vents.insert(i, v);
				}
			}
		}
	}
	println!("{:?}", dangerous);
	
	Ok(())
}

