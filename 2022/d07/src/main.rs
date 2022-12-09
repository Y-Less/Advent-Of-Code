use std::fs::File;
use std::io::prelude::*;

struct Entry<'b>
{
	name: &'b str,
	isDir: bool,
	size: usize,
	entries: Vec<Entry<'b>>,
}

impl<'b> Entry<'b>
{
	fn getSize(&self) -> usize
	{
		if self.isDir
		{
			self.entries.iter().map(|e| e.getSize()).sum()
		}
		else
		{
			self.size
		}
	}
}

fn listDir<'a, 'b>(name: &str, mut lines: impl std::iter::Iterator<Item = &'a str>) -> Entry<'b>
{
	//let x = lines.next();
	//println!("{:?}", x);
	let mut entries: Vec<Entry<'b>> = vec!();
	
	for line in lines
	{
		if line == "$ ls"
		{
			// Ignore this.
			println!("ls");
		}
		else if line == "$ cd .."
		{
			println!("back");
			break;
		}
		if line.chars().nth(0) == Some('$')
		{
			println!("{:?}", line.get(5..));
		}
		else if line.get(0..4) == Some("dir ")
		{
			// Add an entry.
			println!("dir: {:?}", line.get(4..));
		}
		else
		{
			// Add an entry.
			println!("{:?}", line);
			let bits = line.split(' ');
			entries.push(Entry {
				name: bits.nth(1).unwrap(),
				size: bits.nth(0).unwrap().parse::<usize>().expect("Not a number"),
				isDir: false,
				entries: vec!(),
			});
		}
	}
	
	Entry {
		name,
		isDir: true,
		size: 0,
		entries: vec!(),
	}
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	
	let mut lines = input.trim().split('\n');
	lines.next();
	//let x = lines.next();
	//println!("{:?}", x);
	
	listDir("/", lines);
	
	//let mut stack

	Ok(())
}

