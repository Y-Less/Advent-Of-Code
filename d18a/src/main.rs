mod dijkstra;
use crate::dijkstra::*;
use std::vec::Vec;
use std::char;
use std::collections::HashMap;

const INPUT: [&str; 5] = [
	"########################",
	"#...............b.C.D.f#",
	"#.######################",
	"#.....@.a.B.c.d.A.e.F.g#",
	"########################",
];

//const INPUT: [&str; 9] = [
//	"#################",
//	"#i.G..c...e..H.p#",
//	"########.########",
//	"#j.A..b...f..D.o#",
//	"########@########",
//	"#k.E..a...g..B.n#",
//	"########.########",
//	"#l.F..d...h..C.m#",
//	"#################",
//];

//const INPUT: [&str; 81] = [
//	"#################################################################################",
//	"#...........#.......................#...#.........#.....#.....#...Q.............#",
//	"###.#######.#################.#.###.###G#.#####.#.#.#.###.#.#.#.#######.#####.###",
//	"#...#.......................#.#.#.......#...#...#z#.#...#.#.#.#...#u..#....b#...#",
//	"#.#########################.###.#######.###.#.#####.###.#.#.#####.#.#.#####.###.#",
//	"#.............#...#l....#.#...#.....#.#.#...#...#...#...#.#.....#.#.#...#...#...#",
//	"#.###########.#.###.#.#.#.###.#####.#.#.#######.#.###.#N#.###.###.#.###.#####.#.#",
//	"#.#.........#.#.....#.#.....#.#.....#...#.......#.#.#.#.#...#...#.#...#.......#.#",
//	"#.#.#######.#.#.#####.#####.#.#.#####.###.#####.#.#.#.#####.###.#.###.###########",
//	"#.#.#.D.#...#.#.....#.....#.#.#.....#...#.#.....#...#...#...#...#...#...#.......#",
//	"#.#S#.###.###.#####.#####.#.#.#####.###.#.#.#######.###.#.###.#.###.###.#.#####.#",
//	"#...#.......#.#...#.#...#.#.#.#...#.#.#.#.#.#.......#.#.....#.#.#.....#...#.#.A.#",
//	"#.###########.#.#.#.#.###.#.#.#C#.#.#.#.#.#.#.#######.#######.###.#######.#.#.#.#",
//	"#.#.........#.#.#.#.#.#...#.#...#...#.#.#.#.#...#.#.....#.#...#...#...Y.#...#.#.#",
//	"#.#.#######.#.#.###.#.#.#############.#.#.#####.#.#.#.#.#.#.###.###.###.###.#.###",
//	"#.#.#...#.#...#...#...#.........#.....#.#.....#.#...#.#.#.#.......#.#.#.#...#...#",
//	"###.#.#.#.#######.###.#####.###.#.#.###.#.#.#.#.#####.#.#.#########.#.#.#######.#",
//	"#...#.#...#.......#.....#...#.#.#.#.#...#.#.#.#.#.....#...#...#m..#...#.#.....#w#",
//	"#.###.###.#.#.#####.#####.###.#.#.#.#.###.#.#.#.#.#.#######.#.#.#.###.#.#.###.#.#",
//	"#e..#...#...#.#.......#...#.....#.#.#...#.#.#.#...#.#...F...#...#.#...#.#...#...#",
//	"#.#.###.#######.#######.#.#####.###.###.#.#.#.#####.#.###########.#.###.#.#.###.#",
//	"#.#...#...#.......#...#.#.#...#.......#.#.#.#.#.....#...#i..#.I.#.#.#y#.#.#.#.#.#",
//	"#.###.#.#.#.#######.###.###.#.#######.#.#.#.###.#######.#.#.###.#M#.#.#.###.#.#.#",
//	"#.#.#.#.#.......#...#.#.....#...#.....#.#.#...#...#.#...#.#.....#.....#x#...#.#.#",
//	"#.#.#.#.#########.###.#.#######.###.###.#.###.###.#.#.###.#############.#.###.#.#",
//	"#.#.#.#.#.#.....#.#...#...#...#...#...#.#v..#...#...#f..#.#...#..o#.....#.#...#.#",
//	"#.#.#.#.#.#.###.#.#.#.###.#.#####.#####.#.###.#.###.###J#.#.#.#.#.#.#####.###.#.#",
//	"#...#.#...#.#.#...#.#...#.#.#...#.#...#.#.#...#.#...#...#.#.#...#.#.#...#...#...#",
//	"###.#.###.#.#.#####.#####.#.#.#.#.#.#.#.#.#.#####.###.###.#.#####.#.#.###.#.#.###",
//	"#...#...#.#...#...#.........#.#...#.#.#.#.#.#.....#...#.#.#.#...#.X.#.....#.#...#",
//	"#.#####.#####.#.#.#.#########.#####.#.#.#.#.#.#####.###.#.#O#.#.###########.###.#",
//	"#.#...#.....#...#.#.....#...#...#...#.#.#.#...#...#.#..k#...#.#.....#.......#.#.#",
//	"#.#B#######.#####.###.###.#.###.#.###.#.#.#####.#.#.#.###########.#.#.#######.#.#",
//	"#.#.......#.....#.#...#...#...#...#...#.#...#.#.#..j#.....#.......#.#.#.......#.#",
//	"#.#.#####.###.#.#.#####.#####.#.###.#.#.###.#.#.#######.#.#.#######.#.#######.#.#",
//	"#.#...#.....#.#.#.......#.....#.#.#.#.#.#.#.#.#.#...#...#.#...#...#.#.....#...#.#",
//	"#.#####.###.#.###########.#####.#.#.###.#.#.#.#K#.#.#.###R#.#T###.#.#####.#.###.#",
//	"#.#...#...#.#.....#.....#.#...#.#.#...#.#.#.#.#.#.#...#...#.#.#...#...#...#...#.#",
//	"#.#.#.###.#.#####.#.#.###.#.#.#.#.###.#.#.#.#.#.#.#####.#####.#.#####.#.###.#.#.#",
//	"#...#.....#.....#...#.....#.#.......#.........#...#...........#.......H.#...#..p#",
//	"#######################################.@.#######################################",
//	"#...........#.....#.....#...#...#.....#.........#...#.......#...#...#...........#",
//	"#.###.#####.###.#.#.#.#.#.#.#.#.#.#.#.#.#.#.###.#.#.#.#.###.#.#.#.#.###.#######.#",
//	"#.#.#.#...#.....#.#.#.#.#.#.#.#...#.#...#.#.#...#.#...#...#...#...#...#.#.......#",
//	"#.#.#.#.#########.###.#.#.#.#####.#.###.#.#.#.###.#######.###########.#.#.#######",
//	"#.#.#.#.........#...#.#.#.#.....#.#...#.#.#.#...#.......#...#.#...#...#.#.#.....#",
//	"#W#.#.#####.###.###.#.#.#.#####.#####.#.###.###.#######.###.#.#.#.#.###.#.#.###.#",
//	"#...#.#...#...#...#.#.#.#.#.....#.....#.#...#...#........g#.#...#.#.#..s#.#...#.#",
//	"###.#.#.#.###.###.#.#.#.#.#.#####.#####.#.#######.#######.#.#####.#.#.#####.###.#",
//	"#...#.#.#.#t..#...#.#.#.#.#...#...#...#.#.#.......#...#...#.....#.#.#.......#...#",
//	"#.###.#.#.###.#.###.#.#.#.###.###.#.#.#.#.#.#######.#.#########.#.#.#.#######.#.#",
//	"#...#...#...#.#.....#.#.#...#...#.#.#...#...#.......#...#...#...#...#...#.....#.#",
//	"###.#######.#######.#.#.###.###.#.#.#####.###.#########.#.#.#.###.#######.#####.#",
//	"#.#.#.....#...#...#...#.#r#.#.#...#.....#.#.#...#.....#...#.#.#.#...#.....#.....#",
//	"#.#.#.#######.#.#.#####.#.#.#.#########V#.#.###.#.###.#####.#.#.###.#.#####.#####",
//	"#...#.#.....#.#.#.#...#.#.#.#.........#.#...#.#.#...#.#...#...#...#...#.#...#...#",
//	"#P###.#.###.#.#.#.#.#.#.#.#.#.#####.###.###.#.#.###.#.###.#######.#####.#.#####.#",
//	"#.....#.#.....#.#.#.#...#.#.#.....#.....#.#...#...#.#...#.#.............#.......#",
//	"###.###.#####.#.#.###.###.#.#####.#######.###.###.#.###.#.#.#.#########.#######.#",
//	"#...#...#...#.#.#...#.....#.#.#...#.....#.#.....#.#.#.....#.#.........#.....#...#",
//	"#####.###.#.###.###.#######.#.#.###.###.#.#.#####.#.#######.#######.#.#####.#.###",
//	"#.....#...#.......#.#.#.....#.#.#...#...#...#...#.#.......#.#.#...#.#.....#.#.#.#",
//	"#.###.#.###########.#.#.#####.#.###.#.#######.#.#.#.#####.#.#.#.#.#.#####.###.#.#",
//	"#...#.#.#.....#.....#...#.....#...#.#...#...#.#...#...#...#.#...#.#.#...#...#.#d#",
//	"#.#.#.#.#.###.#.#########.#.#####.#.#####.#.#.#########.#.#.#####.#.#.#.###.#.#.#",
//	"#.#.#.#...#.#.#...#.......#.#...#.#.#...#.#.#.........#.#.#.....#.#...#...#.#.#.#",
//	"#.#.#######.#.###.#.#######.#.#.#.#.#.#.#.#.#########.#.#######.#.#########.#.#.#",
//	"#.#.........#...#.#...#.L.#...#c#.#...#.#.#...........#....q..#.#.........#.....#",
//	"#.#######.#####.#.###.###.#####.#.#####.#.#############.#####.#E#########.#######",
//	"#.#.....#.......#...#.....#...#.#.#...#.#.#...#...#.....#...#.#.#.......#.......#",
//	"#.#.###############.#####.###.#.#Z#.#.#.#.#.#.#.#.#.#####.###.#.#.#############.#",
//	"#.#.......#.....#.#.....#.....#...#.#...#...#.#.#...#...#.U.#...#.....#.........#",
//	"#.#.###.#.###.#.#.###.#######.#####.###.#.###.#.#####.#.###.#########.#.#########",
//	"#.#...#.#.....#.....#.......#.#.....#.#.#.#...#...#...#.#.........#...#.#.#.....#",
//	"#.###.#.###########.#######.#.#.#.###.#.#.#.#####.###.#.#.#######.#.#.#.#.#.#.#.#",
//	"#.#...#a#.........#...#.#...#.#.#.#...#.#.#.....#.....#...#.....#...#.#.#...#.#.#",
//	"#.#####.#.#####.#####.#.#.###.###.#.#.#.#.#####.#########.#.###.#######.#####.#.#",
//	"#.#...#.#.#...#.....#...#..n..#...#.#.#.#...#.#.........#.#.#...#.....#.#...#.#.#",
//	"#.#.#.#.#.#.#.#####.###.#######.###.#.#.###.#.#########.###.#.###.###.#.#.#.#.#.#",
//	"#...#...#...#....h#.............#...#...#.............#.....#.......#.....#...#.#",
//	"#################################################################################",
//];

type Pos = (usize, usize);
type Grid = Vec<Vec<u8>>;

fn is_door(ch: u8) -> bool
{
	// 'A' - 'Z'
	ch >= 0x41 && ch <= 0x5A
}

fn is_key(ch: u8) -> bool
{
	// 'a' - 'z'
	ch >= 0x61 && ch <= 0x7A
}

fn is_wall(ch: u8) -> bool
{
	// '#'
	ch == 0x23
}

type NodeList = HashMap<u8, Pos>;

//pub fn build_adjacency(grid: &Vec<Vec<u8>>) -> HashMap<(usize, usize), Vec<((usize, usize), usize)>>
//{
//	let mut adj: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();
//
//
//	adj
//}

fn build_all_links(grid: &Grid, keys: &NodeList, doors: &NodeList)
{
	let mut adj: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();

	// Append the two node type arrays.
	let nodes = doors.clone().drain().fold(keys.clone(), |mut o, c| { o.insert(c.0, c.1); o });

	for (y, row) in grid.iter().enumerate()
	{
		for (x, dot) in row.iter().enumerate()
		{
			if is_wall(*dot)
			{
				continue;
			}
			let mut vec = Vec::new();
			if !is_wall(grid[y + 1][x])
			{
				vec.push(((x, y + 1), 1));
			}
			if !is_wall(grid[y][x + 1])
			{
				vec.push(((x + 1, y), 1));
			}
			if !is_wall(grid[y - 1][x])
			{
				vec.push(((x, y - 1), 1));
			}
			if !is_wall(grid[y][x - 1])
			{
				vec.push(((x - 1, y), 1));
			}
			adj.insert((x, y), vec);
		}
	}

	for from in nodes.iter()
	{
		let adj = dijkstra(*from.1, &adj);
		for to in nodes.iter()
		{
			if from.0 == to.0
			{
				continue;
			}	
			println!("From {} ({:?}) to {} ({:?}) = {:?}", from.0, from.1, to.0, to.1, adj.get(to.1));
		}
	}
}

fn main()
{
	let mut grid = Vec::new();

	let mut keys = HashMap::new();
	let mut doors = HashMap::new();
	let mut start = (0, 0);
	
	for (y, row) in INPUT.iter().enumerate()
	{
		let mut v2 = Vec::new();
		for (x, ch) in row.as_bytes().iter().enumerate()
		{
			if is_door(*ch)
			{
				doors.insert(*ch, (x, y));
			}
			else if is_key(*ch)
			{
				keys.insert(*ch, (x, y));
			}
			if *ch == 0x40
			{
				// Start.
				start = (x, y);
			}
			v2.push(*ch);
		}
		grid.push(v2);
	}

	build_all_links(&grid, &keys, &doors);

//	let ret = test_keys(&keys, &doors, &mut grid, start);
//	//let adj = build_adjacency(&vec);
//	//println!("{:?}", keys.clone());
//	println!("{:?}", ret);
}

