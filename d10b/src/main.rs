use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;

fn gcd(a: i32, b: i32) -> i32
{
	let (mut a, mut b) = if a > b
	{
		(a, b)
	}
	else
	{
		(b, a)
	};

	while b != 0
	{
		let r = a % b;
		a = b;
		b = r;
	}

	a.abs()
}

fn normalise(x: i32, y: i32) -> (i32, i32)
{
	if x == 0
	{
		(0, if y < 0 { -1 } else { 1 })
	}
	else if y == 0
	{
		(if x < 0 { -1 } else { 1 }, 0)
	}
	else
	{
		let gcd = gcd(x, y);
		(x / gcd, y / gcd)
	}
}

fn push_back(quadrant: &mut HashMap<(i32, i32), LinkedList<(i32, i32)>>, pos: (i32, i32), pair: (i32, i32))
{
	let mut nu = LinkedList::<(i32, i32)>::new();
	match quadrant.get(&pos)
	{
		Some(old) =>
		{
			old.iter().for_each(|x| nu.push_back(*x));
		}
		None =>
		{
		}
	}
	nu.push_back(pair);
	quadrant.insert(pos, nu);
}

fn push_front(quadrant: &mut HashMap<(i32, i32), LinkedList<(i32, i32)>>, pos: (i32, i32), pair: (i32, i32))
{
	let mut nu = LinkedList::<(i32, i32)>::new();
	match quadrant.get(&pos)
	{
		Some(old) =>
		{
			old.iter().for_each(|x| nu.push_back(*x));
		}
		None =>
		{
		}
	}
	nu.push_front(pair);
	quadrant.insert(pos, nu);
}

fn main()
{
	let input = [
		[1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1],
		[1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0],
		[0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0],
		[1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
		[0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1],
		[0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0],
		[0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0],
		[0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1],
		[0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1],
		[0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0],
		[0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1],
		[0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1],
		[0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0],
		[1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
		[1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0],
		[0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0],
		[1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0],
		[0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0],
		[1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1],
		[0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
		[1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0],
		[0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1],
		[0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0],
		[0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0],
		[1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0],
		[0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0],
		[0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0],
		[0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0],
		[0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0],
		[0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0],
		[0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1],
		[0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
	];

	// The station is at `(23, 20)`.
	
	// A set includes its zero.
	let mut px_ny = HashMap::<(i32, i32), LinkedList<(i32, i32)>>::new();
	let mut px_py = HashMap::<(i32, i32), LinkedList<(i32, i32)>>::new();
	let mut nx_py = HashMap::<(i32, i32), LinkedList<(i32, i32)>>::new();
	let mut nx_ny = HashMap::<(i32, i32), LinkedList<(i32, i32)>>::new();

	let ax = 23;
	let ay = 33 - 20;

	for (y, inner) in input.iter().enumerate()
	{
		let by = 33 - (y as i32);
		for (x, asteroid) in inner.iter().enumerate()
		{
			let bx = x as i32;
			if *asteroid == 1
			{
				// No branch is true when `ox == 0 && oy == 0`.
				let ox = bx - ax;
				let oy = by - ay;

				let pos = normalise(ox, oy);
				
				if oy > 0 && ox >= 0
				{
					// Upper-right.  Getting closer.
					push_front(&mut px_py, pos, (bx, by));
				}
				if oy <= 0 && ox > 0
				{
					// Lower-right.  Getting further
					push_back(&mut px_ny, pos, (bx, by));
				}
				if oy < 0 && ox <= 0
				{
					// Lower-left.
					push_back(&mut nx_ny, pos, (bx, by));
				}
				if oy >= 0 && ox < 0
				{
					// Upper-left.
					push_front(&mut nx_py, pos, (bx, by));
				}
			}
		}
	}

	let mut px_py: Vec<(f64, LinkedList<(i32, i32)>)> = px_py.drain()
		.map(|(pos, list)| ((pos.0 as f64) / (pos.1 as f64), list))
		.collect();
	let mut px_ny: Vec<(f64, LinkedList<(i32, i32)>)> = px_ny.drain()
		.map(|(pos, list)| ((-pos.1 as f64) / (pos.0 as f64), list))
		.collect();
	let mut nx_ny: Vec<(f64, LinkedList<(i32, i32)>)> = nx_ny.drain()
		.map(|(pos, list)| ((-pos.0 as f64) / (-pos.1 as f64), list))
		.collect();
	let mut nx_py: Vec<(f64, LinkedList<(i32, i32)>)> = nx_py.drain()
		.map(|(pos, list)| ((pos.1 as f64) / (-pos.0 as f64), list))
		.collect();

	px_py.sort_by(|l, r| l.0.partial_cmp(&r.0).unwrap());
	px_ny.sort_by(|l, r| l.0.partial_cmp(&r.0).unwrap());
	nx_ny.sort_by(|l, r| l.0.partial_cmp(&r.0).unwrap());
	nx_py.sort_by(|l, r| l.0.partial_cmp(&r.0).unwrap());
	
	let mut targets = Vec::new();
	
	targets.append(&mut px_py);
	targets.append(&mut px_ny);
	targets.append(&mut nx_ny);
	targets.append(&mut nx_py);
	println!("{:?}", targets);
	
	//let mut targets = targets.iter().cycle();
	//let mut upper = targets.len();
	let mut p = 0;
	for i in 0 .. 200
	{
		//println!("{:?}", targets.next());
		
		//let p = i % targets.len();
		let pair = &targets[p];
		
		println!("{:?}", targets[p]);
		if i == 199
		{
			println!("{:?}", pair.1);
			//let pos = pair.1;
			//println!("{}", pos.0 * 100 + 33 - pos.1);
			p += 1;
		}
		else if pair.1.len() == 1
		{
			targets.remove(p);
		}
		else
		{
			let mut nu = LinkedList::<(i32, i32)>::new();
			pair.1.iter().skip(1).for_each(|x| nu.push_back(*x));
			targets[p] = (pair.0, nu);
			p += 1;
		}
		if p == targets.len()
		{
			p = 0;
		}
	}
}

