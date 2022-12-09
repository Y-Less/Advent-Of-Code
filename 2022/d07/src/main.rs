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

fn listDir<'a>(name: &str, lines: &Vec<&str>, idx: &mut usize) -> Entry
{
	//let x = lines.next();
	//println!("name: {:?}, idx: {:?}", name, idx);
	let mut entries: Vec<Entry> = vec!();
	
	//for line in lines
	loop
	{
		let moo = lines.get(*idx);
		if moo == None
		{
			break;
		}
		let line = *moo.unwrap();
		*idx = *idx + 1;
		if line == "$ ls"
		{
			// Ignore this.
			//println!("ls");
		}
		else if line == "$ cd .."
		{
			//println!("back from {:?}", name);
			break;
		}
		else if line.chars().nth(0) == Some('$')
		{
			//println!("{:?}", line.get(5..));
			entries.push(listDir(line.get(5..).unwrap(), lines, idx));
		}
		else if line.get(0..4) == Some("dir ")
		{
			// Add an entry.
			//println!("dir: {:?}", line.get(4..));
		}
		else
		{
			// Add an entry.
			let mut bits = line.split(' ');
			let a = bits.nth(0).unwrap();
			let b = bits.nth(0).unwrap();
			//println!("file: {:?}, {:?}", b, a);
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
	
	let mut lines = input.trim().split('\n').collect::<Vec<&str>>();
	let mut idx: usize = 1;
	
	let dirs = listDir("/", &lines, &mut idx);
	
	//let mut stack

	Ok(())
}

