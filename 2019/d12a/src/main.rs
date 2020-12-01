//type D = (f64, f64, f64);
#[derive(Debug)]
struct D
{
	x: f64,
	y: f64,
	z: f64,
}

#[derive(Debug)]
struct M
{
	p: D,
	v: D,
}

fn update_v(moons: &mut [M; 4])
{
	//for (i, a) in moons.iter().enumerate()
	for i in 0 .. 4
	{
		//for b in moons.iter().skip(i + 1)
		for j in i + 1 .. 4
		{
			if moons[i].p.x > moons[j].p.x
			{
				moons[i].v.x -= 1.0;
				moons[j].v.x += 1.0;
			}
			if moons[i].p.x < moons[j].p.x
			{
				moons[i].v.x += 1.0;
				moons[j].v.x -= 1.0;
			}
			if moons[i].p.y > moons[j].p.y
			{
				moons[i].v.y -= 1.0;
				moons[j].v.y += 1.0;
			}
			if moons[i].p.y < moons[j].p.y
			{
				moons[i].v.y += 1.0;
				moons[j].v.y -= 1.0;
			}
			if moons[i].p.z > moons[j].p.z
			{
				moons[i].v.z -= 1.0;
				moons[j].v.z += 1.0;
			}
			if moons[i].p.z < moons[j].p.z
			{
				moons[i].v.z += 1.0;
				moons[j].v.z -= 1.0;
			}
		}
	}
}

fn update_p(moons: &mut [M; 4])
{
	for a in moons
	{
		a.p.x += a.v.x;
		a.p.y += a.v.y;
		a.p.z += a.v.z;
	}
}

fn energy(moons: &[M; 4]) -> f64
{
	let mut energy = 0.0;
	for a in moons
	{
		let pot = a.p.x.abs() + a.p.y.abs() + a.p.z.abs();
		let kin = a.v.x.abs() + a.v.y.abs() + a.v.z.abs();
		energy += pot * kin;
	}
	energy
}

fn main()
{
	let mut moons: [M; 4] = [
		M { p: D { x: 14.0, y: 15.0, z:  -2.0, }, v: D { x: 0.0, y: 0.0, z: 0.0 } },
		M { p: D { x: 17.0, y: -3.0, z:   4.0, }, v: D { x: 0.0, y: 0.0, z: 0.0 } },
		M { p: D { x:  6.0, y: 12.0, z: -13.0, }, v: D { x: 0.0, y: 0.0, z: 0.0 } },
		M { p: D { x: -2.0, y: 10.0, z:  -8.0, }, v: D { x: 0.0, y: 0.0, z: 0.0 } },
	];

	//loop
	for i in 0 .. 1000
	{
		update_v(&mut moons);
		update_p(&mut moons);
	}
	println!("{}", energy(&moons));
	//println!("----------");
	//println!("{:?}", moons[0]);
	//println!("{:?}", moons[1]);
	//println!("{:?}", moons[2]);
	//println!("{:?}", moons[3]);
}

