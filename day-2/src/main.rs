use std::io;
//use std::vec;

fn run(prog: &mut Vec<i32>) -> i32
{
	let len = prog.len();
	let mut i = 0;

	loop
	{
		if i >= len
		{
			break;
		}
		
		match prog[i]
		{
			1 =>
			{
				let i0 = prog[i + 3] as usize;
				let i1 = prog[i + 1] as usize;
				let i2 = prog[i + 2] as usize;
				let total = prog[i1] + prog[i2];
				prog[i0] = total;
				i += 4;
			}
			2 =>
			{
				let i0 = prog[i + 3] as usize;
				let i1 = prog[i + 1] as usize;
				let i2 = prog[i + 2] as usize;
				let total = prog[i1] * prog[i2];
				prog[i0] = total;
				i += 4;
			}
			99 => break,
			_ =>
			{
				i += 1;
			}
		}
	}
	prog[0]
}

fn main()
{
    println!("Enter the program.");
	
	let stdin = io::stdin();
	//let mut vec = vec::Vec::new();
	
	let mut input = String::new();
	
	stdin.read_line(&mut input)
		.expect("Please enter something");
	
	let original = input.trim().split(',')
		.map(|x| x.parse().expect("Not a number"))
		.collect::<Vec<i32>>();
	
	let mut noun = 0;
	loop
	{
		let mut verb = 0;
		loop
		{
			let mut prog = original.clone();
			
			prog[1] = noun;
			prog[2] = verb;
			
			let result = run(&mut prog);
			
			if result == 19690720
			{
				println!("Found the result: {}", 100 * noun + verb);
			}
			
			verb += 1;
			if verb == 100
			{
				break;
			}
		}
		
		noun += 1;
		if noun == 100
		{
			break;
		}
	}
	
	
	
	
	//prog.iter().for_each(|x| println!("{}", &x) );
}

