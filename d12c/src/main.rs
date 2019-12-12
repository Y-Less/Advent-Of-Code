use std::thread;

fn gcd(a: u64, b: u64) -> u64
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

	a
}

fn lcd(a: u64, b: u64, c: u64) -> u64
{
	let x = a * b / gcd(a, b);
	x * c / gcd(x, c)
}

fn simulate(p0: [i64; 4], v0: [i64; 4]) -> u64
{
	let mut p = p0;
	let mut v = v0;
	let mut l: u64 = 0;
	loop
	{
		l += 1;
		for i in 0 .. 4
		{
			for j in i + 1 .. 4
			{
				if p[i] > p[j]
				{
					v[i] -= 1;
					v[j] += 1;
				}
				else if p[i] < p[j]
				{
					v[i] += 1;
					v[j] -= 1;
				}
			}
			p[i] += v[i];
		}
		if p == p0 && v == v0
		{
			return l;
		}
	}
}

fn xs() -> u64
{
	let px = [14, 17, 6, -2];
	let vx = [0, 0, 0, 0];
	simulate(px, vx)
}

fn ys() -> u64
{
	let py = [15, -3, 12, 10];
	let vy = [0, 0, 0, 0];
	simulate(py, vy)
}

fn zs() -> u64
{
	let pz = [-2, 4, -13, -8];
	let vz = [0, 0, 0, 0];
	simulate(pz, vz)
}

fn main()
{
	let x = thread::spawn(move || xs());
	let y = thread::spawn(move || ys());
	let z = thread::spawn(move || zs());

	let x = x.join().expect("");
	let y = y.join().expect("");
	let z = z.join().expect("");
	println!("{:?}", x);
	println!("{:?}", y);
	println!("{:?}", z);
	println!("{:?}", lcd(x, y, z));
}

