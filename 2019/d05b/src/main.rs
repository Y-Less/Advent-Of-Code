use std::io;
//use std::vec;

const ADD: u32 = 1;
const MUL: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const JTRUE: u32 = 5;
const JFALSE: u32 = 6;
const LT: u32 = 7;
const EQ: u32 = 8;
const END: u32 = 99;

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
			ADD =>
			{
				let total = get_arg(prog, i, 1, op) + get_arg(prog, i, 2, op);
				let dest = prog[i + 3] as usize;
				prog[dest] = total;
				i += 4;
			}
			MUL =>
			{
				let total = get_arg(prog, i, 1, op) * get_arg(prog, i, 2, op);
				let dest = prog[i + 3] as usize;
				prog[dest] = total;
				i += 4;
			}
			INPUT =>
			{
				let mut input = String::new();
				println!("Please enter a number");
				io::stdin().read_line(&mut input)
					.expect("Please enter a number");
				let total = input.trim().parse()
					.expect("Not a number");
				
				let dest = prog[i + 1] as usize;
				prog[dest] = total;
				i += 2;
			}
			OUTPUT =>
			{
				let total = get_arg(prog, i, 1, op);
				println!("{}", total);
				i += 2;
			}
			JTRUE =>
			{
				let arg = get_arg(prog, i, 1, op);
				let pc = get_arg(prog, i, 2, op);
				if arg != 0
				{
					i = pc as usize;
				}
				else
				{
					i += 3;
				}
			}
			JFALSE =>
			{
				let arg = get_arg(prog, i, 1, op);
				let pc = get_arg(prog, i, 2, op);
				if arg == 0
				{
					i = pc as usize;
				}
				else
				{
					i += 3;
				}
			}
			LT =>
			{
				let arg = get_arg(prog, i, 1, op) < get_arg(prog, i, 2, op);
				let dest = prog[i + 3] as usize;
				if arg
				{
					prog[dest] = 1;
				}
				else
				{
					prog[dest] = 0;
				}
				i += 3;
			}
			EQ =>
			{
				let arg = get_arg(prog, i, 1, op) == get_arg(prog, i, 2, op);
				let dest = prog[i + 3] as usize;
				if arg
				{
					prog[dest] = 1;
				}
				else
				{
					prog[dest] = 0;
				}
				i += 3;
			}
			END => break,
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

