mod intcode;
mod clear;
mod read;
use crate::intcode::Program;
use crate::intcode::ProgramState;
use crate::intcode::ProgramResult;
use std::str;
use std::io::{self, Read};
//extern crate ncurses_rs;
//use ncurses_rs::context::*;

//fn halt(r: ProgramResult, io: bool) -> (bool, i64)
//{
//	match r
//	{
//	ProgramResult::Halt => (true, 0),
//	ProgramResult::Response(v) =>
//		if io { panic!("Expected a request"); } else { (false, v) }
//	ProgramResult::Request =>
//		if io { (false, 0) } else { panic!("Expected a response"); }
//	}
//}
fn draw(grid: [[u8; 40]; 40], score: i64)
{
	//if std::process::Command::new("cls").status().unwrap().success() { }
	crate::clear::clear();
	println!("SCORE: {}", score);
	for i in grid.iter()
	{
		let line = str::from_utf8(i).expect("");
		println!("{}", line);
	}
}

fn run(prog: &mut ProgramState, dir: i64, grid: &mut [[u8; 40]; 40]) -> bool
{
	//let x = halt(prog.step(ProgramResult::Response(dir)), false);
	//let y = halt(prog.step(ProgramResult::Request), false);
	//let z = halt(prog.step(ProgramResult::Request), true);
	//
	//
	let mut s = 0;
	let mut cmd = [0, 0, 0];
	//const TILES: [u8; 5] = [' ' as u8, '█' as u8, '▒' as u8, '─' as u8, '●' as u8];
	const TILES: [u8; 5] = [' ' as u8, '#' as u8, '%' as u8, '-' as u8, 'o' as u8];
	let mut score = 0;
	//let mut first = true;
	let mut res = if dir == 2 { ProgramResult::Request } else { ProgramResult::Response(dir) };
	loop
	{
		match prog.step(res)
		{
		ProgramResult::Response(v) =>
		{
			cmd[s] = v;
			s += 1;
			if s == 3
			{
				if cmd[0] == -1
				{
					score = cmd[2]
				}
				else
				{
					//println!("{} {} {}", cmd[0], cmd[1], cmd[2]);
					grid[cmd[1] as usize][cmd[0] as usize] = TILES[cmd[2] as usize];
				}
				s = 0;
			}
		}
		ProgramResult::Request =>
		{
			draw(*grid, score);
			return true;
		}
		ProgramResult::Halt =>
		{
			draw(*grid, score);
			return false;
		}
		}
		//first = false;
		res = ProgramResult::Request;
	}
}

fn main()
{
//	init();
//	//initscr();
//	//raw();
//	//keypad(stdscr(), true);
//	//noecho();
//	//printw("Press UP to start");
//	//loop
//	//{
//	//	let ch = getch();
//	//	if ch == KEY_UP
//	//	{
//	//		break;
//	//	}
//	//}

	let mut buffer: [u8; 3] = [0, 0, 0];
	let mut grid: [[u8; 40]; 40] = [[' ' as u8; 40]; 40];
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut prog = ProgramState::load("../inputs/d13.txt");
	run(&mut prog, 2, &mut grid);

	loop
	{
		println!("Hit enter to continue");
		handle.read(&mut buffer).expect("");
		println!("{}", buffer[0]);
		if !run(&mut prog, 2, &mut grid)
		{
			break;
		}
	}
}
