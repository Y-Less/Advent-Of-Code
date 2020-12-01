#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct D
{
	x: i32,
	y: i32,
	z: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct M
{
	p: D,
	v: D,
}

fn update(moons: &mut [M; 4])
{
	for i in 0 .. 4
	{
		for j in i + 1 .. 4
		{
			if moons[i].p.x > moons[j].p.x
			{
				moons[i].v.x -= 1;
				moons[j].v.x += 1;
			}
			else if moons[i].p.x < moons[j].p.x
			{
				moons[i].v.x += 1;
				moons[j].v.x -= 1;
			}
			if moons[i].p.y > moons[j].p.y
			{
				moons[i].v.y -= 1;
				moons[j].v.y += 1;
			}
			else if moons[i].p.y < moons[j].p.y
			{
				moons[i].v.y += 1;
				moons[j].v.y -= 1;
			}
			if moons[i].p.z > moons[j].p.z
			{
				moons[i].v.z -= 1;
				moons[j].v.z += 1;
			}
			else if moons[i].p.z < moons[j].p.z
			{
				moons[i].v.z += 1;
				moons[j].v.z -= 1;
			}
		}
		moons[i].p.x += moons[i].v.x;
		moons[i].p.y += moons[i].v.y;
		moons[i].p.z += moons[i].v.z;
	}
}

//fn energy(moons: &[M; 4]) -> i32
//{
//	let mut energy = 0;
//	for a in moons
//	{
//		let pot = a.p.x.abs() + a.p.y.abs() + a.p.z.abs();
//		let kin = a.v.x.abs() + a.v.y.abs() + a.v.z.abs();
//		energy += pot * kin;
//	}
//	energy
//}

fn main()
{
	// State after 0 steps:
	let e0 = 0;
	let s0: [M; 4] = [
		M { p: D { x: 14, y: 15, z:  -2, }, v: D { x: 0, y: 0, z: 0 } },
		M { p: D { x: 17, y: -3, z:   4, }, v: D { x: 0, y: 0, z: 0 } },
		M { p: D { x:  6, y: 12, z: -13, }, v: D { x: 0, y: 0, z: 0 } },
		M { p: D { x: -2, y: 10, z:  -8, }, v: D { x: 0, y: 0, z: 0 } },
	];

//	// State after 4294967296 steps:
//	let e1 = 224399;
//	let s1: [M; 4] = [
//		M { p: D { x: -108, y: 373, z: 697 }, v: D { x: -39, y: 1, z: -39 } },
//		M { p: D { x: 119, y: -714, z: -583 }, v: D { x: 2, y: -33, z: 1 } },
//		M { p: D { x: 76, y: 461, z: -398 }, v: D { x: 13, y: 29, z: 28 } },
//		M { p: D { x: -52, y: -86, z: 265 }, v: D { x: 24, y: 3, z: 10 } },
//	];
//    
//	// State after 4294967296 steps:
//	let e2 = 171738;
//	let s2: [M; 4] = [
//		M { p: D { x: -427, y: 460, z: -1206 }, v: D { x: -13, y: -2, z: -28 } },
//		M { p: D { x: 92, y: 251, z: 383 }, v: D { x: 2, y: 4, z: 36 } },
//		M { p: D { x: 25, y: -437, z: 266 }, v: D { x: 18, y: 0, z: -20 } },
//		M { p: D { x: 345, y: -240, z: 538 }, v: D { x: -7, y: -2, z: 12 } },
//	];

	let mut moons = s0;

	//let mut energies: [i32; 4294967296] = [0; 4294967296];

	//loop
	//for i in 0 .. 4294967296_u64 * 4
	let mut i: u64 = 0;
	loop
	{
		if i % 1000000000 == 0
		{
			println!("{}", i);
		}
		i += 1;
		update(&mut moons);
		if moons == s0
		{
			println!("{}", i);
			break;
		}
		//if energy(&moons) == e1 && 
		//if moons == s1
		//{
		//	println!("----------");
		//	println!("{} = {}", i, e1);
		//	println!("{:?}", moons[0]);
		//	println!("{:?}", moons[1]);
		//	println!("{:?}", moons[2]);
		//	println!("{:?}", moons[3]);
		//}
	}
//	println!("----------");
//	println!("{}", energy(&moons));
//	println!("{:?}", moons[0]);
//	println!("{:?}", moons[1]);
//	println!("{:?}", moons[2]);
//	println!("{:?}", moons[3]);
//
//	let cmp = moons;
//	let mut moons = s0;
//
//	for i in 0 .. 4294967296_u64 * 4
//	{
//		if i % 100000000 == 0
//		{
//			println!("{}", i);
//		}
//		update(&mut moons);
//		//if energy(&moons) == e1 && 
//		if moons == cmp
//		{
//			println!("----------");
//			println!("{} = {}", i + 4294967296_u64, e2);
//			println!("{:?}", moons[0]);
//			println!("{:?}", moons[1]);
//			println!("{:?}", moons[2]);
//			println!("{:?}", moons[3]);
//		}
//	}
}

