use std::io;
//use std::vec;

fn get_arg(prog: &Vec<i32>, base: usize, arg: usize, op: u32) -> i32
{
	let div = 10_u32.pow(arg as u32 + 1);
	match op / div % 10
	{
		1 => prog[(base + arg) as usize],
		_ => prog[prog[(base + arg) as usize] as usize],
	}
}

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
		
		let op = prog[i] as u32;
		match op % 100
		{
			1 =>
			{
				let total = get_arg(prog, i, 1, op) + get_arg(prog, i, 2, op);
				let dest = prog[i + 3] as usize;
				prog[dest] = total;
				i += 4;
			}
			2 =>
			{
				let total = get_arg(prog, i, 1, op) * get_arg(prog, i, 2, op);
				let dest = prog[i + 3] as usize;
				prog[dest] = total;
				i += 4;
			}
			3 =>
			{
				let mut input = String::new();
				io::stdin().read_line(&mut input)
					.expect("Please enter a number");
				let total = input.trim().parse()
					.expect("Not a number");
				
				let dest = prog[i + 1] as usize;
				prog[dest] = total;
				i += 2;
			}
			4 =>
			{
				let total = get_arg(prog, i, 1, op);
				println!("{}", total);
				i += 2;
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
	
	let mut prog = original.clone();
	
	run(&mut prog);
}

