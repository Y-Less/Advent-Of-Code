fn pair(a: u32, b: u32, c: u32, d: u32) -> bool
{
	a != b && b == c && c != d
}

fn valid(num: u32) -> bool
{
	let a = num / 100000 % 10;
	let b = num / 10000 % 10;
	let c = num / 1000 % 10;
	let d = num / 100 % 10;
	let e = num / 10 % 10;
	let f = num / 1 % 10;

	
	let paired =
		pair(10, a, b, c) ||
		pair(a, b, c, d) ||
		pair(b, c, d, e) ||
		pair(c, d, e, f) ||
		pair(d, e, f, 0);

	let increasing = (a <= b) && (b <= c) && (c <= d) && (d <= e) && (e <= f);

	paired && increasing
}

fn main()
{
	let mut count = 0;
	for i in 156218..652527
	{
		if valid(i)
		{
			count = count + 1;
		}
	}
	println!("{}", count);
}

