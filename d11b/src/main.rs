mod intcode;
use crate::intcode::Program;
use crate::intcode::ProgramState;
use crate::intcode::ProgramResult;
use std::collections::LinkedList;
use std::str;

// x, y, order, first (here), white.
type Step = (i32, i32, i32, bool, bool);

// Angle:
//
//      0
//      |
//  3 --+-- 1
//      |
//      2
//
fn update_pos(x: &mut i32, y: &mut i32, a: &mut i32, right: bool)
{
	if right
	{
		*a = (*a + 1) % 4;
	}
	else
	{
		*a = (*a + 3) % 4;
	}
	match *a
	{
	0 => *y = *y + 1,
	1 => *x = *x + 1,
	2 => *y = *y - 1,
	3 => *x = *x - 1,
	_ => {}
	}
}

fn visited<'a>(steps: impl Iterator<Item = &'a Step>, x: i32, y: i32) -> bool
{
	for i in steps
	{
		if i.0 == x && i.1 == y
		{
			return true;
		}
	}
	return false;
}

fn colour<'a>(steps: impl Iterator<Item = &'a Step>, x: i32, y: i32) -> i64
{
	for i in steps
	{
		if i.0 == x && i.1 == y //&& i.4
		{
			return if i.4 { 1 } else { 0 };
		}
	}
	return 0;
}

fn halt(r: ProgramResult, io: bool) -> (bool, i64)
{
	match r
	{
	ProgramResult::Halt => (true, 0),
	ProgramResult::Response(v) =>
		if io { panic!("Expected a request"); } else { (false, v) }
	ProgramResult::Request =>
		if io { (false, 0) } else { panic!("Expected a response"); }
	}
}

fn main()
{
	let mut prog = ProgramState::load("../inputs/d11.txt");
	halt(prog.step(ProgramResult::Request), true);

	let mut x = 0;
	let mut y = 0;
	let mut a = 0;

	let mut steps = LinkedList::new();
	steps.push_back((0, 0, 0, true, true));
	//steps.push_back((6, 0, 0, true));
	//steps.push_back((1, 1, 0, true));
	//
	//println!("{}", visited(&steps, 0, 0));
	//println!("{}", visited(&steps, 0, 5));
	//println!("{}", visited(&steps, 1, 1));
	//println!("{}", visited(&steps, 6, 0));

	let mut idx = 1;
	loop
	{
		// Input the current colour.
		let r = halt(prog.step(ProgramResult::Response(
			colour(steps.iter().rev(), x, y)
		)), false);
		if r.0
		{
			break;
		}
		//println!("{:?}", colour(steps.iter(), x, y));
		steps.push_back((x, y, idx, !visited(steps.iter(), x, y), r.1 == 1));
		//println!("{:?}", steps);
		let r = halt(prog.step(ProgramResult::Request), false);
		if r.0
		{
			break;
		}
		//println!("{:?}", r);
		update_pos(&mut x, &mut y, &mut a, r.1 == 1);
		let r = halt(prog.step(ProgramResult::Request), true);
		if r.0
		{
			break;
		}
		//	break;
		idx += 1;
	}

	const C0: u8 = ' ' as u8;
	const C1: u8 = '#' as u8;
	let mut grid = [[C0; 200]; 200];
	//let mut idx = 0;
	//let mut count = 0;
	for i in steps.iter()
	{
		grid[(i.1 + 100) as usize][(i.0 + 100) as usize] =
			if i.4 { C1 }
			else { C0 }
		//println!("({}, {}) = {}", i.0, i.1, i.4);
		//if !visited(steps.iter().take(idx), i.0, i.1)
		//{
		//	count += 1;
		//}
		//idx += 1;
	}
	for i in grid.iter()
	{
		println!("{:?}", str::from_utf8(i));
	}
	
	//println!("{}", count);
}

