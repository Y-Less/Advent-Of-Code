use std::io;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Direction
{
	U { n: i32 },
	D { n: i32 },
	L { n: i32 },
	R { n: i32 },
}

type Cell = (i32, i32);

fn print_direction(i: &Direction) -> ()
{
	match i
	{
	Direction::U { n } => println!("U {{ {} }}", n),
	Direction::D { n } => println!("D {{ {} }}", n),
	Direction::L { n } => println!("L {{ {} }}", n),
	Direction::R { n } => println!("R {{ {} }}", n),
	}
}

fn print_position(i: &Cell) -> ()
{
	println!("( {} {} )", i.0, i.1);
}

fn get_direction(i: &str) -> Direction //Result<Direction, &'static str>
{
	match &i[0..1]
	{
	"U" => Direction::U { n: i[1..].parse().expect("Not a number") },
	"D" => Direction::D { n: i[1..].parse().expect("Not a number") },
	"L" => Direction::L { n: i[1..].parse().expect("Not a number") },
	"R" => Direction::R { n: i[1..].parse().expect("Not a number") },
	"u" => Direction::U { n: i[1..].parse().expect("Not a number") },
	"d" => Direction::D { n: i[1..].parse().expect("Not a number") },
	"l" => Direction::L { n: i[1..].parse().expect("Not a number") },
	"r" => Direction::R { n: i[1..].parse().expect("Not a number") },
	_ => panic!("Not a valid direction"),
	}
}

fn insert_all(cells: &mut HashSet<Cell>, a: Cell, b: Cell) -> Cell
{
	if a.1 < b.1
	{
		for y in a.1 .. b.1
		{
			cells.insert((a.0, y + 1));
		}
	}
	if a.1 > b.1
	{
		for y in b.1 .. a.1
		{
			cells.insert((a.0, y));
		}
	}
	if a.0 < b.0
	{
		for x in a.0 .. b.0
		{
			cells.insert((x + 1, a.1));
		}
	}
	if a.0 > b.0
	{
		for x in b.0 .. a.0
		{
			cells.insert((x, a.1));
		}
	}
	b
}

fn check_all(cells: &HashSet<Cell>, intersects: &mut HashSet<Cell>, a: Cell, b: Cell) -> Cell
{
	if a.1 < b.1
	{
		for y in a.1 .. b.1
		{
			if cells.contains(&(a.0, y + 1))
			{
				intersects.insert((a.0, y + 1));
			}
		}
	}
	if a.1 > b.1
	{
		for y in b.1 .. a.1
		{
			if cells.contains(&(a.0, y))
			{
				intersects.insert((a.0, y));
			}
		}
	}
	if a.0 < b.0
	{
		for x in a.0 .. b.0
		{
			if cells.contains(&(x + 1, a.1))
			{
				intersects.insert((x + 1, a.1));
			}
		}
	}
	if a.0 > b.0
	{
		for x in b.0 .. a.0
		{
			if cells.contains(&(x, a.1))
			{
				intersects.insert((x, a.1));
			}
		}
	}
	b
}

//fn distance(a: Cell, b: Cell) -> i32
//{
//	(a.0 - b.0).abs() + (a.1 - b.1).abs()
//}

fn distance(a: &Cell) -> i32
{
	a.0.abs() + a.1.abs()
}

fn main()
{
	println!("Enter the two paths.");
	
	let stdin = io::stdin();
	
	let mut i0 = String::new();
	
	stdin.read_line(&mut i0)
		.expect("Please enter the first wire");
	
	let wire1 = i0.trim().split(',');

	let mut cells: HashSet<Cell> = HashSet::new();
	
	let mut pos: Cell = (0, 0);
	cells.insert(pos);

	wire1.for_each(|x|
	{
		match get_direction(x)
		{
		Direction::U { n } => pos = insert_all(&mut cells, pos, (pos.0, pos.1 + n)),
		Direction::D { n } => pos = insert_all(&mut cells, pos, (pos.0, pos.1 - n)),
		Direction::L { n } => pos = insert_all(&mut cells, pos, (pos.0 - n, pos.1)),
		Direction::R { n } => pos = insert_all(&mut cells, pos, (pos.0 + n, pos.1)),
		}
	});
	//cells.iter().for_each(print_position);
	
	let mut i1 = String::new();

	stdin.read_line(&mut i1)
		.expect("Please enter the second wire");

	let wire2 = i1.trim().split(',');
	pos = (0, 0);

	let mut intersects: HashSet<Cell> = HashSet::new();

	wire2.for_each(|x|
	{
		match get_direction(x)
		{
		Direction::D { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0, pos.1 - n)),
		Direction::U { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0, pos.1 + n)),
		Direction::L { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0 - n, pos.1)),
		Direction::R { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0 + n, pos.1)),
		}
	});
	intersects.remove(&(0, 0));
	println!("Minimum: {:?}", intersects.iter().map(distance).min());
}

