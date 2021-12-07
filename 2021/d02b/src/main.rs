use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d02.txt")?;
		file.read_to_string(&mut input)?;
	}

	//let course = input.trim().split('\n')
	//	.collect::<Vec<String>>();

	let mut f = 0;
	let mut d = 0;
	let mut a = 0;
	for i in input.trim().split('\n')
	{
		let mut iter = i.splitn(2, ' ');
		let command = iter.next().unwrap();
		let value: i32 = iter.next().unwrap().parse().expect("Not a number");
		//println!("{:?}", command);
		//println!("{:?}", value);
		if (command == "forward")
		{
			f = f + value;
			d = d + (a * value);
		}
		else if (command == "down")
		{
			//d = d + value;
			a = a + value;
		}
		else if (command == "up")
		{
			//u = u + value;
			a = a - value;
		}
	}
	
	//let mut prev = -1;
	//let mut count = -1;
    //
	//for n in numbers
	//{
	//	if (n > prev)
	//	{
	//		count = count + 1;
	//	}
	//	prev = n;
	//}

	println!("{:?}", f * d);

	Ok(())
}

