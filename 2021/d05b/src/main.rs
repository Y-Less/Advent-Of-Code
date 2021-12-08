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
			let x = parts[0];
			let y = parts[1];
			let z = std::cmp::max(parts[0], parts[2]) - std::cmp::min(parts[0], parts[2]);
			for n in 0 .. z + 1
			{
				let i = x + (if (parts[0] > parts[2]) {-n} else {n});
				let j = y + (if (parts[1] > parts[3]) {-n} else {n});
				//println!("{:?} {:?}", i, j);
				if let Some(v) = vents.get_mut(&i)
				{
					if (v.contains_key(&j))
					{
						if (v[&j] == 1)
						{
							dangerous = dangerous + 1;
						}
						v.insert(j, v[&j] + 1);
					}
					else
					{
						v.insert(j, 1);
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
		else
		{
			for i in std::cmp::min(parts[0], parts[2])..std::cmp::max(parts[0], parts[2]) + 1
			{
				for j in std::cmp::min(parts[1], parts[3])..std::cmp::max(parts[1], parts[3]) + 1
				{
					//println!("{:?} {:?}", i, j);
					if let Some(v) = vents.get_mut(&i)
					{
						if (v.contains_key(&j))
						{
							if (v[&j] == 1)
							{
								dangerous = dangerous + 1;
							}
							v.insert(j, v[&j] + 1);
						}
						else
						{
							v.insert(j, 1);
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
	}
	println!("{:?}", dangerous);
	
	Ok(())
}

