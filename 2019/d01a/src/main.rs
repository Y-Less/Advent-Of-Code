use std::io;
use std::vec;

fn main()
{
    println!("Enter the numbers.");
	
	let stdin = io::stdin();
	let mut vec = vec::Vec::new();
	
	loop
	{
		let mut num = String::new();
		
		stdin.read_line(&mut num)
			.expect("Please enter something");
		let num: i32 = match num.trim().parse()
		{
			Ok(n) => n,
			Err(_) => break,
		};
		
			//.expect("Bye");
		
		vec.push(num);
		
		//let calc: i32 = num / 3 - 2;
		//println!("Number: {}, {}", num, calc);
	}
	
	//vec.for_each(|x| println!("{}", x));
	let total: i32 = vec.iter().fold(0, |cur, x| cur + (x / 3 - 2));
	
	println!("{}", total);
	
}

