mod intcode;
mod clear;
mod read;
use crate::intcode::Program;
use crate::intcode::ProgramState;
use crate::intcode::ProgramResult;
use std::str;

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

type Pos = (usize, usize);

fn run(prog: &mut ProgramState, dir: i64, grid: &mut [[u8; 40]; 40], score: &mut i64, ball: Pos, paddle: Pos) -> (bool, Pos, Pos)
{
	let mut ball = ball;
	let mut paddle = paddle;
	let mut s = 0;
	let mut cmd = [0, 0, 0];
	const TILES: [u8; 5] = [' ' as u8, '#' as u8, '%' as u8, '-' as u8, 'o' as u8];
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
					*score = cmd[2]
				}
				else
				{
					//println!("{} {} {}", cmd[0], cmd[1], cmd[2]);
					grid[cmd[1] as usize][cmd[0] as usize] = TILES[cmd[2] as usize];
					if cmd[2] == 4
					{
						ball = (cmd[0] as usize, cmd[1] as usize);
					}
					if cmd[2] == 3
					{
						paddle = (cmd[0] as usize, cmd[1] as usize);
					}
				}
				s = 0;
			}
		}
		ProgramResult::Request =>
		{
			draw(*grid, *score);
			return (true, ball, paddle);
		}
		ProgramResult::Halt =>
		{
			draw(*grid, *score);
			return (false, ball, paddle);
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

	let mut grid: [[u8; 40]; 40] = [[' ' as u8; 40]; 40];
	let mut score = 0;
	//let stdin = io::stdin();
	//let mut handle = stdin.lock();

	let mut prog = ProgramState::load("../inputs/d13.txt");
	
	let mut ball: Pos = (0, 0);
	let mut paddle: Pos = (0, 0);
	let res = run(&mut prog, 2, &mut grid, &mut score, ball, paddle);
	ball = res.1;
	paddle = res.2;

	loop
	{
		let dir =
			if ball.0 < paddle.0 { -1 }
			else if ball.0 > paddle.0 { 1 }
			else { 0 };
		let res = run(&mut prog, dir, &mut grid, &mut score, ball, paddle);
		if !res.0
		{
			break;
		}
		ball = res.1;
		paddle = res.2;
	}
}

//49
//	28
//77
//	17
//94