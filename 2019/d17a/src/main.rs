mod intcode;
use crate::intcode::Program;
use crate::intcode::ProgramState;
use std::vec::Vec;
use std::str;

fn main()
{
	
	let mut prog = ProgramState::load("../inputs/d17.txt");

	let mut s = Vec::new();

	loop
	{
		let (a, b) = run!(prog);
		if a
		{
			break;
		}
		s.push(b as u8);
	}

	println!("{}", str::from_utf8(&s).expect("").to_string());
    println!("Hello, world!");
}
