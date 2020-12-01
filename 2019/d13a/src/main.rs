mod intcode;
use crate::intcode::Program;
use crate::intcode::ProgramState;
use crate::intcode::ProgramResult;
use std::str;

fn main()
{
	let mut prog = ProgramState::load("../inputs/d13.txt");
	let grid = prog.run();

	for i in grid.iter()
	{
		let line = str::from_utf8(i).expect("");
		println!("{}", line);
		line.iter().
	}
}
