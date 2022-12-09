use std::fs::File;
use std::io::prelude::*;

struct Entry
{
	name: String,
	isDir: bool,
	size: usize,
	entries: Vec<Entry>,
}

impl Entry
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

fn listDir<'a>(name: &str, mut lines: impl std::iter::Iterator<Item = &'a str>) -> Entry
{
	//let x = lines.next();
	//println!("{:?}", x);
	let mut entries: Vec<Entry> = vec!();
	
	//for line in lines
	loop
	{
		let line = lines.next().unwrap();
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
		else if line.chars().nth(0) == Some('$')
		{
			println!("{:?}", line.get(5..));
			entries.push(listDir(line.get(5..).unwrap(), lines));
		}
		else if line.get(0..4) == Some("dir ")
		{
			// Add an entry.
			println!("dir: {:?}", line.get(4..));
			//entries.push(listDir(line.get(4..).unwrap(), lines));
		}
		else
		{
			// Add an entry.
			let mut bits = line.split(' ');
			let a = bits.nth(0).unwrap();
			let b = bits.nth(0).unwrap();
			println!("file: {:?}, {:?}", b, a);
			entries.push(Entry {
				name: b.to_string(),
				size: a.parse::<usize>().expect("Not a number"),
				isDir: false,
				entries: vec!(),
			});
		}
	}
	
	Entry {
		name: name.to_string(),
		isDir: true,
		size: 0,
		entries,
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

