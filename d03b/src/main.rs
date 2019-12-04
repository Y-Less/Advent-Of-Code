use std::io;
use std::collections::HashMap;

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

fn print_position(i: (&Cell, &i32)) -> ()
{
	println!("({}, {}) = {}", (i.0).0, (i.0).1, i.1);
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

fn maybe_insert(map: &mut HashMap<Cell, i32>, key: Cell, ai: i32, bi: i32)
{
	match map.get(&key)
	{
        Some(distance) =>
		{
			if ai + bi < *distance
			{
				map.insert(key, ai + bi);
			}
		}
        None =>
		{
			map.insert(key, ai + bi);
		}
    }
}

fn insert_all(cells: &mut HashMap<Cell, i32>, a: Cell, b: Cell, ai: &mut i32) -> Cell
{
	if a.1 < b.1
	{
		for y in a.1 .. b.1
		{
			*ai = *ai + 1;
			maybe_insert(cells, (a.0, y + 1), *ai, 0);
		}
	}
	if a.1 > b.1
	{
		*ai = *ai + a.1 - b.1 + 1;
		for y in b.1 .. a.1
		{
			*ai = *ai - 1;
			maybe_insert(cells, (a.0, y), *ai, 0);
		}
		*ai = *ai + a.1 - b.1 - 1;
	}
	if a.0 < b.0
	{
		for x in a.0 .. b.0
		{
			*ai = *ai + 1;
			maybe_insert(cells, (x + 1, a.1), *ai, 0);
		}
	}
	if a.0 > b.0
	{
		*ai = *ai + a.0 - b.0 + 1;
		for x in b.0 .. a.0
		{
			*ai = *ai - 1;
			maybe_insert(cells, (x, a.1), *ai, 0);
		}
		*ai = *ai + a.0 - b.0 - 1;
	}
	b
}

fn check_all(cells: &HashMap<Cell, i32>, intersects: &mut HashMap<Cell, i32>, a: Cell, b: Cell, bi: &mut i32) -> Cell
{
	if a.1 < b.1
	{
		for y in a.1 .. b.1
		{
			*bi = *bi + 1;
			let key = (a.0, y + 1);
			match cells.get(&key)
			{
				Some(ai) => maybe_insert(intersects, key, *ai, *bi),
				None => {}
			}
		}
	}
	if a.1 > b.1
	{
		*bi = *bi + a.1 - b.1 + 1;
		for y in b.1 .. a.1
		{
			*bi = *bi - 1;
			let key = (a.0, y);
			match cells.get(&key)
			{
				Some(ai) => maybe_insert(intersects, key, *ai, *bi),
				None => {}
			}
		}
		*bi = *bi + a.1 - b.1 - 1;
	}
	if a.0 < b.0
	{
		for x in a.0 .. b.0
		{
			*bi = *bi + 1;
			let key = (x + 1, a.1);
			match cells.get(&key)
			{
				Some(ai) => maybe_insert(intersects, key, *ai, *bi),
				None => {}
			}
		}
	}
	if a.0 > b.0
	{
		*bi = *bi + a.0 - b.0 + 1;
		for x in b.0 .. a.0
		{
			*bi = *bi - 1;
			let key = (x, a.1);
			match cells.get(&key)
			{
				Some(ai) => maybe_insert(intersects, key, *ai, *bi),
				None => {}
			}
		}
		*bi = *bi + a.0 - b.0 - 1;
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

	let mut cells: HashMap<Cell, i32> = HashMap::new();
	let mut ai = 0;
	
	let mut pos: Cell = (0, 0);
	cells.insert(pos, 0);


	wire1.for_each(|x|
	{
		match get_direction(x)
		{
		Direction::U { n } => pos = insert_all(&mut cells, pos, (pos.0, pos.1 + n), &mut ai),
		Direction::D { n } => pos = insert_all(&mut cells, pos, (pos.0, pos.1 - n), &mut ai),
		Direction::L { n } => pos = insert_all(&mut cells, pos, (pos.0 - n, pos.1), &mut ai),
		Direction::R { n } => pos = insert_all(&mut cells, pos, (pos.0 + n, pos.1), &mut ai),
		}
	});
	//cells.iter().for_each(print_position);
	
	let mut i1 = String::new();
    
	stdin.read_line(&mut i1)
		.expect("Please enter the second wire");
    
	let wire2 = i1.trim().split(',');
	pos = (0, 0);
    
	let mut intersects: HashMap<Cell, i32> = HashMap::new();
	let mut bi = 0;
    
	wire2.for_each(|x|
	{
		match get_direction(x)
		{
		Direction::D { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0, pos.1 - n), &mut bi),
		Direction::U { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0, pos.1 + n), &mut bi),
		Direction::L { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0 - n, pos.1), &mut bi),
		Direction::R { n } => pos = check_all(&cells, &mut intersects, pos, (pos.0 + n, pos.1), &mut bi),
		}
	});
	intersects.remove(&(0, 0));
	println!("Minimum: {:?}", intersects.iter().map(|x| x.1).min());
}

