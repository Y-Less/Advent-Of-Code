mod intcode;
mod clear;
mod dijkstra;
//mod read;
use crate::intcode::Program;
use crate::intcode::ProgramState;
use crate::intcode::ProgramResult;
use std::str;
//use crate::read::read;
//extern crate nalgebra;
//extern crate dijkstra;
//
//use nalgebra::*;
use crate::dijkstra::*;
use std::collections::HashMap;
use std::vec::Vec;
use dijkstra::Vertex;

type Pos = (usize, usize);

const DIM: usize = 42;
const WHOLE: usize = DIM * DIM;	
const START: Pos = (DIM / 2, DIM / 2);

fn draw(grid: [[u8; DIM]; DIM], bot: Pos)
{
	crate::clear::clear();
	for (i, x) in grid.iter().enumerate()
	{
		if i == bot.1
		{
			let mut line = str::from_utf8(x).expect("").to_string();
			line.replace_range(bot.0 .. bot.0 + 1, "D");
			println!("{}", line);
		}
		else
		{
			let line = str::from_utf8(x).expect("");
			println!("{}", line);
		}
	}
}

fn run(prog: &mut ProgramState, dir: i64, grid: &mut [[u8; DIM]; DIM], bot: Pos, end: &mut Pos) -> (bool, Pos)
{
	let mut bot = bot;
	const TILES: [u8; 4] = [' ' as u8, '#' as u8, '.' as u8, 'o' as u8];
	let mut res = if dir == 0 { ProgramResult::Request } else { ProgramResult::Response(dir) };
	loop
	{
		match prog.step(res)
		{
		ProgramResult::Response(v) =>
		{
			match v
			{
			0 =>
			{
				match dir
				{
				1 => grid[bot.1 - 1][bot.0] = TILES[1],
				2 => grid[bot.1 + 1][bot.0] = TILES[1],
				3 => grid[bot.1][bot.0 - 1] = TILES[1],
				4 => grid[bot.1][bot.0 + 1] = TILES[1],
				_ => {}
				}
			}
			1 =>
			{
				match dir
				{
				1 => bot.1 -= 1,
				2 => bot.1 += 1,
				3 => bot.0 -= 1,
				4 => bot.0 += 1,
				_ => {}
				}
				grid[bot.1][bot.0] = TILES[2];
			}
			2 =>
			{
				match dir
				{
				1 => bot.1 -= 1,
				2 => bot.1 += 1,
				3 => bot.0 -= 1,
				4 => bot.0 += 1,
				_ => {}
				}
				grid[bot.1][bot.0] = TILES[3];
				*end = (bot.0, bot.1);
				//draw(*grid, bot);
				//return (true, bot);
			}
			_ => {}
			}
		}
		ProgramResult::Request =>
		{
			//draw(*grid, bot);
			return (true, bot);
		}
		ProgramResult::Halt =>
		{
			//draw(*grid, bot);
			return (false, bot);
		}
		}
		res = ProgramResult::Request;
	}
}

fn main()
{
	let mut grid: [[u8; DIM]; DIM] = [[' ' as u8; DIM]; DIM];

	let mut prog = ProgramState::load("../inputs/d15.txt");
	
	let mut bot: Pos = START;
	let mut end: Pos = (0, 0);
	run(&mut prog, 0, &mut grid, bot, &mut end);
	let mut dir = 1;

	let mut i = 0;
	loop
	{
		let res = run(&mut prog, dir, &mut grid, bot, &mut end);
		if !res.0
		{
			break;
		}
		match dir
		{
		1 => if grid[bot.1 - 1][bot.0] == '#' as u8 { dir = 4; } else { dir = 3; },
		2 => if grid[bot.1 + 1][bot.0] == '#' as u8 { dir = 3; } else { dir = 4; },
		3 => if grid[bot.1][bot.0 - 1] == '#' as u8 { dir = 1; } else { dir = 2; },
		4 => if grid[bot.1][bot.0 + 1] == '#' as u8 { dir = 2; } else { dir = 1; },
		_ => {}
		}
		bot = res.1;
		i += 1;
		if i % 100000 == 0
		{
			break;
		}
		//if i % 10000 == 0
		//{
		//	draw(grid, bot);
		//}
	}
	draw(grid, (DIM / 2, DIM / 2));
	// We now have a grid.  Build a very very simple matrix of single length node connections.
	
	//let no_connection = i32::max_value();
	
	//let mut matrix: [[i32; WHOLE]; WHOLE] = [[i32::max_value(); WHOLE]; WHOLE];
	//let template = vec![i32::max_value(); WHOLE];
	//let mut matrix: Vec<Vec<i32>> = vec![template.clone(); WHOLE];
	//let template = vec![i32::max_value(); WHOLE];
//	let mut matrix: Vec<i32> = vec![i32::max_value(); WHOLE * WHOLE];

	const DOT: u8 = '.' as u8;
	grid[START.1][START.0] = DOT; // Ensure we can get from the start point.
	grid[end.1][end.0] = DOT; // Ensure we can get to the end point.
	
	let mut adj: HashMap<Vertex, Vec<(Vertex, usize)>> = HashMap::new();

	for (y, row) in grid.iter().enumerate()
	{
		if y == 0 || y == DIM - 1
		{
			continue;
		}
		for (x, dot) in row.iter().enumerate()
		{
			if *dot != DOT
			{
				continue;
			}
			if x == 0 || x == DIM - 1
			{
				continue;
			}
			//let name = ;
			let mut vec = Vec::new();
//			let base = (y * DIM + x) * WHOLE;
//			if grid[y - 1][x] == DOT
//			{
//				matrix[base + (y - 1) * DIM + x] = 1;
//			}
			if grid[y + 1][x] == DOT
			{
				vec.push((Vertex::new((x, y + 1)), 1));
			}
			if grid[y][x + 1] == DOT
			{
				vec.push((Vertex::new((x + 1, y)), 1));
				//matrix[base + (y + 1) * DIM + x] = 1;
			}
			if grid[y - 1][x] == DOT
			{
				vec.push((Vertex::new((x, y - 1)), 1));
			}
			if grid[y][x - 1] == DOT
			{
				vec.push((Vertex::new((x - 1, y)), 1));
				//matrix[base + (y + 1) * DIM + x] = 1;
			}
			adj.insert(Vertex::new((x, y)), vec);
//			if grid[y][x - 1] == DOT
//			{
//				matrix[base + y * DIM + (x - 1)] = 1;
//			}
//			if grid[y][x + 1] == DOT
//			{
//				matrix[base + y * DIM + (x + 1)] = 1;
//			}
		}
	}

//	let matrix = DMatrix::from_row_vector(WHOLE, WHOLE, &matrix);
//	println!("{:?}", START);
//	println!("{:?}", end);
//	let start = START.1 * DIM + START.0;
//	let end = end.1 * DIM + end.0;
//	println!("{:?}", start);
//	println!("{:?}", end);
	//println!("{:?}", matrix);


//	let path = dijkstra_path(&matrix, start, end);
//	println!("{:?}", path);
//
//
//	let path = dijkstra_table_gen(&matrix, start);
//	print_matrix(&path.0, start);
//	print_matrix(&path.0, end);
	let ret = dijkstra(Vertex::new((START.1, START.0)), &adj);
	println!("{:?}", ret);
	println!("{:?}", ret.get(&Vertex::new((end.1, end.0))));
}

//fn print_matrix(matrix: &DMatrix<i32>, row: usize) -> ()
//{
//	println!("");
//	let i: i32 = i32::max_value();
////	for row in 0 .. matrix.nrows()
//	{
//		for item in matrix.row(row).iter()
//		{
//			if *item == i
//			{
//				print!("{}", "-");
//			}
//			else
//			{
//				print!("{}", item);
//			};
//		}
//		println!("");
//	}
//}

